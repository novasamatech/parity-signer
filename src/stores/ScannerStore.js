// @flow
import { Container } from 'unstated';
import transaction from '../util/transaction';
import { keccak, ethSign, brainWalletSign, decryptData } from '../util/native';
import { type Account } from './AccountsStore';

type TXRequest = Object;

type ScannerState = {
  txRequest: TXRequest | null,
  scanErrorMsg: string
};

export default class ScannerStore extends Container<ScannerState> {
  state = {
    txRequest: null,
    tx: '',
    dataToSign: '',
    signedData: '',
    scanErrorMsg: ''
  };

  async setTXRequest(txRequestData) {
    const txRequest = JSON.parse(txRequestData);
    const tx = await transaction(txRequest.data.rlp);
    const dataToSign = await keccak(txRequest.data.rlp);
    this.setState({
      txRequest,
      tx,
      dataToSign
    });
  }

  async signData(account: Account, pin = '1') {
    let seed = await decryptData(account.encryptedSeed, pin);
    this.setState({
      signedData: await brainWalletSign(seed, this.state.dataToSign)
    });
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
