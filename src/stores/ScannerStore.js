// Copyright 2015-2019 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

// @flow

import Payload from '@polkadot/api/SignerPayload';
import { encodeAddress } from '@polkadot/util-crypto';
import { Container } from 'unstated';
import { NETWORK_LIST, NetworkProtocols, EthereumNetworkKeys } from '../constants';
import { saveTx } from '../util/db';
import { brainWalletSign, decryptData, keccak, ethSign } from '../util/native';
import transaction from '../util/transaction';
import { rawDataToU8A } from '../util/rawDataToU8A';
import { Account } from './AccountsStore';

type TXRequest = Object;

type SignedTX = {
  recipient: Account,
  sender: Account,
  txRequest: TXRequest,
};

type ScannerState = {
  dataToSign: string,
  message: string,
  multipartData: any,
  recipient: Account,
  scanErrorMsg: string,
  sender: Account,
  signedData: string,
  signedTxList: [SignedTX],
  tx: Object,
  txRequest: TXRequest | null,
  type: 'transaction' | 'message',
  unsignedData: any
};

const defaultState = {
  busy: false,
  dataToSign: '',
  message: null,
  multipartData: {},
  recipient: null,
  scanErrorMsg: '',
  sender: null,
  signedData: '',
  tx: '',
  txRequest: null,
  type: null,
  unsignedData: {}
};

export default class ScannerStore extends Container<ScannerState> {
  state = defaultState;

  parseRawData(rawData) {
    const bytes = rawDataToU8A(rawData);
    const hex = bytes.map(byte => byte.toString(16));
    const uosAfterFrames = hex.slice(5); // FIXME handle multipart

    const zerothByte = uosAfterFrames[0];
    const firstByte = uosAfterFrames[1];
    const secondByte = uosAfterFrames[2];
    let action;
    let address;
    let data = {};
    data['data'] = {}; // for consistency with legacy data format.

    try {
      debugger;
      // decode payload appropriately via UOS
      switch (zerothByte) {
        case 45: // Ethereum UOS payload
          action = firstByte === 0 || firstByte === 2 ? 'signData' : firstByte === 1 ? 'signTransaction' : null;
          address = uosAfterFrames.slice(2, 22);

          data['action'] = action;
          data['data']['account'] = account;

          if (action === 'signData') {
            data['data']['rlp'] = uosAfterFrames[13];
          } else if (action === 'signTransaction') {
            data['data']['data'] = rawAfterFrames[13];
          } else {
            throw new Error('Could not determine action type.');
          }
          break;
        case 53: // Substrate UOS payload
          const crypto = firstByte === 0 ? 'ed25519' : firstByte === 1 ? 'sr25519' : null;
          action = secondByte === 0 || secondByte === 1 ? 'signData': secondByte === 2 || secondByte === 3 ? 'signTransaction' : null;

          const publicKeyAsBytes = uosAfterFrames.slice(3, 35);
          const ss58Encoded = encodeAddress(publicKeyAsBytes);
          const encryptedData: Uint8Array = uosAfterFrames.slice(35);

          debugger;

          data['action'] = action;
          data['data']['crypto'] = crypto;
          data['data']['account'] = ss58Encoded;

          debugger;

          switch(secondByte) {
            case 0:
              if (encryptedData.length > 256) {
                data['oversized'] = true; // flag and warn that we are signing the hash because payload was too big.
                data['isHash'] = true; // flag and warn that signing a hash is inherently dangerous
                data['data']['data'] = blake2b(data.data.payload); // FIXME: use native blake2b function
              } else {
                data['isHash'] = false;
                data['data']['data'] = Payload(encryptedData);
              }
              break;
            case 1:
              data['isHash'] = true;
              data['data']['data'] = Payload(encryptedData);
              break;
            case 2:
              data['isHash'] = false;
              data['data']['data'] = Payload(encryptedData);
              break;
            case 3: // Cold Signer should attempt to decode message to utf8
              data['data']['data'] = decodeToString(encryptedData);
              break;
            default:
              break;
          }
          break;
        default:
          throw new Error('we cannot handle the payload: ', rawData);
      }

      this.setState({
        unsignedData: data
      });
    } catch (e) {
      scannerStore.setBusy();
      throw new Error('we cannot handle the payload: ', rawData);
    }
  }

  setPartData(frame, frameCount, partData, accountsStore) {
    debugger;
    if (partData[0] === new Uint8Array([0x00]) || partData[0] === new Uint8Array([0x7B])) {
      // part_data for frame 0 MUST NOT begin with byte 00 or byte 7B.
      throw new Error('Error decoding invalid part data.');
    }

    // we havne't filled all the frames yet
    if (Object.keys(this.state.multipartData.length) < frameCount) {
      const nextDataState = this.state.multipartData;
      
      nextDataState[frame] = partData;

      this.setState({
        multipartData: nextDataState
      });
    }

    // all the frames are filled
    if (Object.keys(this.state.multipartData.length) === frameCount) {
      const concatMultipartData = Object.keys(this.state.multipartData).reduce((result, data) => res.concat(this.state.multipartData[data]));
      const data = this.parseRawData(data, accountsStore);
      this.setData(data);
    }
  }

  async setData(accountsStore) {
    // - Cold Signer SHOULD (at the user's discretion) sign the message, immortal_payload, or payload if payload is of length 256 bytes or fewer.
    switch (data.action) {
      case 'signTransaction':
        return await this.setTXRequest(this.state.unsignedData, accountsStore);
      case 'signData':
        return await this.setDataToSign(this.state.unsignedData, accountsStore);
      default:
        throw new Error(
          `Scanned QR should contain either transaction or a message to sign`
        );
    }
  }

  async setDataToSign(signRequest, accountsStore) {
    const message = signRequest.data.data;
    const address = signRequest.data.account;
    const crypto = signRequest.data.crypto;

    if (crypto === 'sr25519' || crypto === 'ed25519') { // only Substrate payload has crypto field
      const substrateSign = async () => { /* Placeholder function for now */ return message; }
      const dataToSign = await substrateSign(message);

    } else {
      const dataToSign = await ethSign(message);
    }

    const sender = accountsStore.getByAddress(address);

    if (!sender || !sender.encryptedSeed) {
      throw new Error(
        `No private key found for ${address} found in your signer key storage.`
      );
    }
    this.setState({
      type: 'message',
      sender,
      message,
      dataToSign
    });
    return true;
  }

  async setTXRequest(txRequest, accountsStore) {
    this.setBusy();

    const protocol = txRequest.data.data.crypto ? NetworkProtocols.SUBSTRATE : NetworkProtocols.ETHEREUM

    if (protocol === NetworkProtocols.ETHEREUM && !(txRequest.data && txRequest.data.rlp && txRequest.data.account)) {
      throw new Error(`Scanned QR contains no valid transaction`);
    }

    if (protocol === NetworkProtocols.ETHERUEM) {
      const tx = await transaction(txRequest.data.rlp);
      const { ethereumChainId = 1 } = tx;
      const networkKey = ethereumChainId;
    }

     // TODO cater for Substrate
    const sender = accountsStore.getById({
      protocol,
      networkKey: networkKey || NetworkProtocols.SUBSTRATE,
      address: txRequest.data.account
    });
    const networkTitle = NETWORK_LIST[networkKey].title;

    if (!sender || !sender.encryptedSeed) {
      throw new Error(
        `No private key found for account ${
          txRequest.data.account
        } found in your signer key storage for the ${networkTitle} chain.`
      );
    }

    // TODO cater for Substrate
    const recipient = accountsStore.getById({
      protocol: NetworkProtocols.ETHEREUM,
      networkKey: tx.ethereumChainId,
      address: tx.action
    });
    const dataToSign = await keccak(txRequest.data.rlp);
    this.setState({
      type: 'transaction',
      sender,
      recipient,
      txRequest,
      tx,
      dataToSign
    });
    return true;
  }

  async signData(pin = '1') {
    const { type, sender } = this.state;
    const seed = await decryptData(sender.encryptedSeed, pin);
    const signedData = await brainWalletSign(seed, this.state.dataToSign);
    this.setState({ signedData });
    if (type == 'transaction') {
      await saveTx({
        hash: this.state.dataToSign,
        tx: this.state.tx,
        sender: this.state.sender,
        recipient: this.state.recipient,
        signature: signedData,
        createdAt: new Date().getTime()
      });
    }
  }

  getType() {
    return this.state.type;
  }

  setBusy() {
    this.setState({
      busy: true
    });
  }

  setReady() {
    this.setState({
      busy: false
    });
  }

  isBusy() {
    return this.state.busy;
  }

  cleanup() {
    this.setState(defaultState);
  }

  getSender() {
    return this.state.sender;
  }

  getRecipient() {
    return this.state.recipient;
  }

  getTXRequest() {
    return this.state.txRequest;
  }

  getMessage() {
    return this.state.message;
  }

  getTx() {
    return this.state.tx;
  }

  getDataToSign() {
    return this.state.dataToSign;
  }

  getSignedTxData() {
    return this.state.signedData;
  }

  setErrorMsg(scanErrorMsg) {
    this.setState({ scanErrorMsg });
  }

  getErrorMsg() {
    return this.state.scanErrorMsg;
  }
}
