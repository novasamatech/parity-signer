// Copyright 2015-2017 Parity Technologies (UK) Ltd.
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
import transaction from '../util/transaction';
import { keccak, ethSign, brainWalletSign, decryptData } from '../util/native';
import { saveTx, loadAccountTxs } from '../util/db';
import { accountId } from '../util/account';``
import { NETWORK_TITLES, NETWORK_IDS } from '../constants';
import { type Account } from './AccountsStore';

type TXRequest = Object;

type SignedTX = {
  txRequest: TXRequest,
  sender: Account,
  recipient: Account
};

type ScannerState = {
  txRequest: TXRequest | null,
  tx: Object,
  sender: Account,
  recipient: Account,
  dataToSign: string,
  signedData: string,
  scanErrorMsg: string,
  signedTxList: [SignedTX]
};

const defaultState = {
  busy: false,
  txRequest: null,
  sender: null,
  recipient: null,
  tx: '',
  dataToSign: '',
  signedData: '',
  scanErrorMsg: ''
};

export default class ScannerStore extends Container<ScannerState> {
  state = defaultState;

  async setTXRequest(txRequest, accountsStore) {
    this.setBusy();
    if (!(txRequest.data && txRequest.data.rlp && txRequest.data.account)) {
      throw new Error(
        `Scanned QR contains no valid transaction`
      );
    }
    const tx = await transaction(txRequest.data.rlp);
    const { chainId = '1' } = tx;

    const sender = accountsStore.getById({
      networkType: 'ethereum',
      chainId,
      address: txRequest.data.account
    });
    const networkTitle = NETWORK_TITLES[chainId];

    if (!sender.encryptedSeed) {
      throw new Error(
        `No private key found for account ${
          txRequest.data.account
        } found in your signer key storage for the ${networkTitle} chain.`
      );
    }

    const recipient = accountsStore.getById({
      networkType: 'ethereum',
      chainId: tx.chainId,
      address: tx.action
    });
    const dataToSign = await keccak(txRequest.data.rlp);
    this.setState({
      sender,
      recipient,
      txRequest,
      tx,
      dataToSign
    });
    return true;
  }

  async signData(pin = '1') {
    const sender = this.state.sender;
    const seed = await decryptData(sender.encryptedSeed, pin);
    const signedData = await brainWalletSign(seed, this.state.dataToSign);
    this.setState({ signedData });
    await saveTx({
      hash: this.state.dataToSign,
      tx: this.state.tx,
      sender: this.state.sender,
      recipient: this.state.recipient,
      signature: signedData,
      createdAt: new Date().getTime()
    });
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
