// @flow
import { Container } from 'unstated'
import { saveAccount, deleteAccount } from '../util/db'
import { encryptData, decryptData } from '../util/native'

type Account = {
  name: string,
  address: string,
  seed: string,
  encryptedSeed: string,
}

type AccountsState = {
  accounts: [Account],
  newAccount: Account,
  selected: string,
};

export default class AccountsStore extends Container<AccountsState> {
  state = {
    accounts: [],
    newAccount: {
      name: '',
      address: '',
      seed: '',
      encryptedSeed: ''
    },
    selected: ''
   };

  constructor(props) {
    super(props)
  }

  select(address) {
    this.setState({selected: address})
  }

  update(accountUpdate: {address: string}) {
    let account = this.getByAddress(accountUpdate.address)
    if (!account) {
      account = this.state.newAccount
    }
    Object.assign(account, accountUpdate)
    this.setState()
  }

  updateSelected() {
    this.update(this.getSelected())
  }

  // TODO: PIN
  async saveSelected() {
    try {
      const account = this.getSelected()
      if (!account) {
        return
      }
      let encryptedSeed = await encryptData(account.seed, '')
      delete account.seed
      saveAccount({
        ...account,
        encryptedSeed
      })
    } catch (e) {
      console.error(e)
    }
  }

  getByAddress(address: string): ?Account {
    return this.state.newAccount.address === address && this.state.newAccount
     || this.state.accounts.find(account => account.address === address)
  }

  getSelected(): Account {
    // console.log(this.state.selected, this.state.newAccount);
    return this.getByAddress(this.state.selected) || {
      name: '',
      address: '',
      seed: '',
      encryptedSeed: ''
    }
  }
}
