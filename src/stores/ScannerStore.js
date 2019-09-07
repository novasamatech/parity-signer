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
import { GenericExtrinsicPayload } from '@polkadot/types';
import { hexStripPrefix, isU8a, u8aToHex } from '@polkadot/util';
import { checkAddress, decodeAddress, encodeAddress  } from '@polkadot/util-crypto';
import { Container } from 'unstated';

import { NETWORK_LIST, NetworkProtocols, SUBSTRATE_NETWORK_LIST } from '../constants';
import { saveTx } from '../util/db';
import { isAscii } from '../util/message';
import { blake2s, brainWalletSign, decryptData, keccak, ethSign, substrateSign } from '../util/native';
import transaction from '../util/transaction';
import { constructDataFromBytes, asciiToHex } from '../util/decoders';
import { Account } from './AccountsStore';

type TXRequest = Object;

type SignedTX = {
  recipient: Account,
  sender: Account,
  txRequest: TXRequest,
};

type ScannerState = {
  dataToSign: string,
  isHash: boolean,
  isOversized: boolean,
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
  isHash: false,
  isOversized: false,
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
  unsignedData: null
};

export default class ScannerStore extends Container<ScannerState> {
  state = defaultState;

  async setUnsigned(data) {
    this.setState({
      unsignedData: JSON.parse(data)
    });
  }

  async setParsedData(strippedData, accountsStore) {
    const parsedData = await constructDataFromBytes(strippedData);

    if (!accountsStore.getByAddress(parsedData.data.account)) {
      let networks = Object.keys(SUBSTRATE_NETWORK_LIST);

      for (let i = 0; i < networks.length; i++) {
        let key =  networks[i];
        let account = accountsStore.getByAddress(encodeAddress(decodeAddress(parsedData.data.account), SUBSTRATE_NETWORK_LIST[key].prefix));

        if (account) {
          parsedData['data']['account'] = account.address;
          break;
        }
      }
    }

    if (parsedData.isMultipart) {
      this.setPartData(parsedData.frame, parsedData.frameCount, parsedData.partData, accountsStore);
      return;
    }

    this.setState({
      unsignedData: parsedData
    });
  }

  async setPartData(frame, frameCount, partData, accountsStore) {
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
      // fixme: this needs to be concated to a binary blob
      const concatMultipartData = Object.keys(this.state.multipartData).reduce((result, data) => res.concat(this.state.multipartData[data]));
      const data = this.setParsedData(concatMultipartData);
      this.setData(data);
    }
  }

  async setData(accountsStore) {
    switch (this.state.unsignedData.action) {
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
    const address = signRequest.data.account;
    const crypto = signRequest.data.crypto;
    const message = signRequest.data.data;
    const isHash = signRequest.isHash;
    const isOversized = signRequest.oversized;

    let dataToSign = '';

    if (crypto === 'sr25519' || crypto === 'ed25519') { // only Substrate payload has crypto field
      dataToSign = message;
    } else {
      dataToSign = await ethSign(message);
    }

    const sender = accountsStore.getByAddress(address);

    if (!sender || !sender.encryptedSeed) {
      throw new Error(
        `No private key found for ${address} found in your signer key storage.`
      );
    }

    this.setState({
      dataToSign,
      isHash,
      isOversized,
      message,
      sender,
      type: 'message',
    });
    return true;
  }

  async setTXRequest(txRequest, accountsStore) {
    this.setBusy();

    const isOversized = txRequest.oversized;

    const protocol = txRequest.data.rlp ? NetworkProtocols.ETHEREUM : NetworkProtocols.SUBSTRATE
    const isEthereum = protocol === NetworkProtocols.ETHEREUM;

    if (isEthereum && !(txRequest.data && txRequest.data.rlp && txRequest.data.account)) {
      throw new Error(`Scanned QR contains no valid transaction`);
    }

    const tx = isEthereum ? await transaction(txRequest.data.rlp) : txRequest.data.data;
    const networkKey = isEthereum ? tx.ethereumChainId : txRequest.data.data.genesisHash.toHex();

    const sender = accountsStore.getById({
      networkKey,
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

    const recipient = accountsStore.getById({
      networkKey: networkKey,
      address: isEthereum ? tx.action : txRequest.data.account
    });

    // For Eth, always sign the keccak hash.
    // For Substrate, only sign the blake2 hash if payload bytes length > 256 bytes (handled in decoder.js).
    const dataToSign = isEthereum ? await keccak(txRequest.data.rlp) : txRequest.data.data;

    this.setState({
      type: 'transaction',
      sender,
      recipient,
      txRequest,
      tx,
      dataToSign,
      isOversized
    });
    return true;
  }

  async signData(pin = '1') {
    const { isHash, sender, type } = this.state;

    const seed = await decryptData(sender.encryptedSeed, pin);
    const isEthereum = NETWORK_LIST[sender.networkKey].protocol === NetworkProtocols.ETHEREUM;

    let signedData;

    if (isEthereum) {
      signedData = await brainWalletSign(seed, this.state.dataToSign);
    } else {
      let signable;

      if (this.state.dataToSign instanceof GenericExtrinsicPayload) {
        signable = u8aToHex(this.state.dataToSign.toU8a(true), -1, false);
      } else if (isU8a(this.state.dataToSign)) {
        signable = hexStripPrefix(u8aToHex(this.state.dataToSign));
      } else if (isAscii(this.state.dataToSign)) {
        signable = hexStripPrefix(asciiToHex(this.state.dataToSign));
      }

      signedData = await substrateSign(seed, signable);
    }

    this.setState({ signedData });

    if (type === 'transaction') {
      await saveTx({
        hash: (isEthereum || isHash) ? this.state.dataToSign : await blake2s(this.state.dataToSign.toHex()),
        tx: this.state.tx,
        sender,
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

  getIsHash() {
    return this.state.isHash;
  }

  getIsOversized() {
    return this.state.isOversized;
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

  getUnsigned() {
    return this.state.unsignedData;
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
