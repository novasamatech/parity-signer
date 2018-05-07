// @flow
import { Container } from 'unstated'
import { loadAccounts, saveAccount, deleteAccount } from '../util/db'
import { encryptData, decryptData } from '../util/native'

type TXRequest = Object

type ScannerState = {
  txRequest: TXRequest | null,
  scanErrorMsg: string
};

export default class ScannerStore extends Container<AccountsState> {

  state = {
    txRequest: null,
    scanErrorMsg: ''
  }

  setTXRequest(scannedTX) {
    this.setState({scannedTX})
  }

  getTXRequest() {
    return this.state.scannedTX
  }

  setErrorMsg(scanErrorMsg) {
    this.setState({ scanErrorMsg })
  }

  getErrorMsg() {
    return this.state.scanErrorMsg
  }
}
