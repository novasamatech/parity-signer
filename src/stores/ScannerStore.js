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
import { Container } from 'unstated';
import { NETWORK_LIST, NetworkProtocols, EthereumNetworkKeys } from '../constants';
import { saveTx } from '../util/db';
import { brainWalletSign, decryptData, keccak, ethSign } from '../util/native';
import transaction from '../util/transaction';
import { Account } from './AccountsStore';

type TXRequest = Object;

type SignedTX = {
  txRequest: TXRequest,
  sender: Account,
  recipient: Account
};

type ScannerState = {
  type: 'transaction' | 'message',
  txRequest: TXRequest | null,
  message: string,
  tx: Object,
  sender: Account,
  recipient: Account,
  dataToSign: string,
  signedData: string,
  scanErrorMsg: string,
  signedTxList: [SignedTX],
  unsignedData: {}
};

const defaultState = {
  type: null,
  busy: false,
  txRequest: null,
  message: null,
  multipartData: {},
  sender: null,
  recipient: null,
  tx: '',
  dataToSign: '',
  signedData: '',
  scanErrorMsg: ''
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

          data['action'] = action;
          data['data']['crypto'] = crypto;
          data['data']['account'] = ss58Encoded;

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

  async setData(data, accountsStore) {
    // - Cold Signer SHOULD (at the user's discretion) sign the message, immortal_payload, or payload if payload is of length 256 bytes or fewer.
    switch (data.action) {
      case 'signTransaction':
        return await this.setTXRequest(data, accountsStore);
      case 'signData':
        return await this.setDataToSign(data, accountsStore);
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


  /*
  Example Full Raw Data
  ---
  4 // indicates binary
  37 // indicates data length
  0000 // frame count
  0100 // first frame
  --- UOS Specific Data
  53 // indicates payload is for Substrate
  01 // crypto: sr25519
  00 // indicates action: signData
  f4cd755672a8f9542ca9da4fbf2182e79135d94304002e6a09ffc96fef6e6c4c // public key
  544849532049532053504152544121 // actual payload message to sign (should be SCALE)
  0 // terminator
  --- SQRC Filler Bytes
  ec11ec11ec11ec // SQRC filler bytes
  */
  function rawDataToU8A(rawData) {
    if (!rawData) {
      return null;
    }

    // Strip filler bytes padding at the end
    if (rawData.substr(-2) === 'ec') {
      rawData = rawData.substr(0, rawData.length - 2);
    }

    while (rawData.substr(-4) === 'ec11') {
      rawData = rawData.substr(0, rawData.length - 4);
    }

    // Verify that the QR encoding is binary and it's ending with a proper terminator
    if (rawData.substr(0, 1) !== '4' || rawData.substr(-1) !== '0') {
      return null;
    }

    // Strip the encoding indicator and terminator for ease of reading
    rawData = rawData.substr(1, rawData.length - 2);

    const length8 = parseInt(rawData.substr(0, 2), 16) || 0;
    const length16 = parseInt(rawData.substr(0, 4), 16) || 0;
    let length = 0;

    // Strip length prefix
    if (length8 * 2 + 2 === rawData.length) {
      rawData = rawData.substr(2);
      length = length8;
    } else if (length16 * 2 + 4 === rawData.length) {
      rawData = rawData.substr(4);
      length = length16;
    } else {
      return null;
    }

    const bytes = new Uint8Array(length);

    for (let i = 0; i < length; i++) {
      bytes[i] = parseInt(rawData.substr(i * 2, 2), 16);
    }

    return bytes;
  }
}
