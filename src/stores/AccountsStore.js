// @flow
import { Container } from 'unstated'
import { loadAccounts, saveAccount, deleteAccount } from '../util/db'
import { encryptData, decryptData } from '../util/native'

export type Account = {
  name: string,
  address: string,
  seed: string,
  encryptedSeed: string,
  createdAt: number,
  updatedAt: number,
  archived: boolean
}

type AccountsState = {
  accounts: Map<string, Account>,
  newAccount: Account,
  selected: string,
};

function empty(address = '') {
  return {
    name: '',
    address,
    seed: '',
    createdAt: (new Date).getTime(),
    updatedAt: (new Date).getTime(),
    archived: false,
    encryptedSeed: null
  }
}

export default class AccountsStore extends Container<AccountsState> {
  state = {
    accounts: new Map(),
    newAccount: empty(),
    selected: ''
   };

  constructor(props) {
    super(props)
    this.refreshList()
  }

  select(address) {
    this.setState({selected: address.toLowerCase()})
  }

  updateNew(accountUpdate: Object) {
    Object.assign(this.state.newAccount, accountUpdate)
    this.setState({})
  }

  getNew(): Account {
    return this.state.newAccount
  }

  submitNew() {
    this.setState({
      accounts:
        this.state.accounts.set(this.state.newAccount.address.toLowerCase(), this.state.newAccount)})
  }

  update(accountUpdate: {address: string}) {
    let account = this.state.accounts.get(accountUpdate.address.toLowerCase())
    if (!account) {
      this.state.accounts.set(accountUpdate.address.toLowerCase(), accountUpdate)
      account = this.state.accounts.get(accountUpdate.address.toLowerCase())
    }
    Object.assign(account, accountUpdate)
    this.setState({})
  }

  updateSelected(accountUpdate) {
    this.update(Object.assign(this.getSelected(), accountUpdate))
  }

  async refreshList() {
    loadAccounts().then((res) => {
      const accounts = new Map(res.map(a => [a.address.toLowerCase(), a]))
      this.setState({accounts})
    })
  }

  async save(account, pin = null) {
    try {
      if (pin && account.seed) {
        let encryptedSeed = await encryptData(account.seed, pin)
        delete account.seed
        account.encryptedSeed = encryptedSeed
      }
      account.updatedAt = (new Date).getTime()
      saveAccount(account)
    } catch (e) {
      console.error(e)
    }
  }

  async deleteAccount(account) {
    // deleteAccount(account)
    account.archived = true
    this.state.accounts.set(account.address.toLowerCase(), account)
    this.setState({
      accounts:this.state.accounts
    })
    await this.save(account)
  }

  async saveSelected(pin) {
    await this.save(this.getSelected(), pin)
  }

  async checkPinForSelected(pin) {
    const account = this.getSelected()
    if (account && account.encryptedSeed) {
      return await decryptData(account.encryptedSeed, pin)
    } else {
      return false
    }
  }

  getByAddress(address: string): ?Account {
    return this.state.accounts.get(address.toLowerCase()) || empty(address)
  }

  getSelected(): ?Account {
    return this.state.accounts.get(this.state.selected)
  }

  getAccounts(): Array<Account> {
    return Array.from(this.state.accounts.values()).filter(a => !a.archived)
  }
}
