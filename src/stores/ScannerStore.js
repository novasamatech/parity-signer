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
  signedTxList: [SignedTX]
};

const defaultState = {
  type: null,
  busy: false,
  txRequest: null,
  message: null,
  sender: null,
  recipient: null,
  tx: '',
  dataToSign: '',
  signedData: '',
  scanErrorMsg: ''
};

export default class ScannerStore extends Container<ScannerState> {
  state = defaultState;

  async setData(data, accountsStore) {
    console.log('setData => ', data);
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

    console.log('address => ', address);
    debugger;

    if (crypto === 'sr25519' || crypto === 'ed25519') { // only Substrate payload has crypto field
      const substrateSign = async () => { /* Placeholder function for now */ return message; }
      const dataToSign = await substrateSign(message);

    } else {
      const dataToSign = await ethSign(message);
    }

    const sender = accountsStore.getByAddress(address);

    debugger;
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

    if (!(txRequest.data && txRequest.data.rlp && txRequest.data.account)) {
      throw new Error(`Scanned QR contains no valid transaction`);
    }

    const protocol = txRequest.data.data.crypto ? NetworkProtocols.SUBSTRATE : NetworkProtocols.ETHEREUM

    console.log(txRequest);
    debugger;

    const tx = await transaction(txRequest.data.rlp);
    const { ethereumChainId = 1 } = tx;
    const networkKey = ethereumChainId;

     // TODO cater for Substrate
    const sender = accountsStore.getById({
      protocol: NetworkProtocols.ETHEREUM,
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
