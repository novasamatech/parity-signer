// @flow
import { Container } from 'unstated'
import { loadAccounts, saveAccount, deleteAccount } from '../util/db'
import { encryptData, decryptData } from '../util/native'

export type Account = {
  name: string,
  address: string,
  seed: string,
  encryptedSeed: string,
  archived: boolean
}

type AccountsState = {
  accounts: Map<string, Account>,
  newAccount: Account,
  selected: string,
};

export default class AccountsStore extends Container<AccountsState> {
  state = {
    accounts: new Map(),
    newAccount: {
      name: '',
      address: '',
      seed: '',
      archived: false,
      encryptedSeed: null
    },
    selected: ''
   };

  constructor(props) {
    super(props)
    this.refreshList()
  }

  select(address) {
    this.setState({selected: address})
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
        this.state.accounts.set(this.state.newAccount.address, this.state.newAccount)})
  }

  update(accountUpdate: {address: string}) {
    let account = this.state.accounts.get(accountUpdate.address)
    if (!account) {
      this.state.accounts.set(accountUpdate.address, accountUpdate)
      account = this.state.accounts.get(accountUpdate.address)
    }
    Object.assign(account, accountUpdate)
    this.setState({})
  }

  updateSelected(accountUpdate) {
    this.update(Object.assign(this.getSelected(), accountUpdate))
  }

  async refreshList() {
    loadAccounts().then((res) => {
      const accounts = new Map(res.map(a => [a.address, a]))
      this.setState({accounts})
    })
  }

  async save(account, pin = null) {
    try {
      if (pin && account.seed) {
        let encryptedSeed = await encryptData(account.seed, pin)
        delete account.seed
        account.encryptedSeed = encryptedSeed
        saveAccount(account)
      } else {
        saveAccount(account)
      }

    } catch (e) {
      console.error(e)
    }
  }

  async deleteAccount(account) {
    // deleteAccount(account)
    account.archived = true
    this.state.accounts.set(account.address, account)
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
    console.log(account)
    console.log(this.state)
    if (account && account.encryptedSeed) {
      return await decryptData(account.encryptedSeed, pin)
    } else {
      return false
    }
  }

  getByAddress(address: string): ?Account {
    return this.state.accounts.get(address)
  }

  getSelected(): ?Account {
    return this.state.accounts.get(this.state.selected)
  }

  getAccounts(): Array<Account> {
    return Array.from(this.state.accounts.values()).filter(a => !a.archived)
  }
}
