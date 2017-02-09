import { ADD_ACCOUNT } from '../constants/AccountActions'

const initialAccounts = [{
  address: '0xb794f5ea0ba39494ce839613fffba74279579268',
}, {
  address: '0xe853c56864a2ebe4576a807d26fdc4a0ada51919',
}, {
  address: '0x53d284357ec70ce289d6d64134dfac8e511c8a3d',
}, {
  address: '0xd56d423cdc0e437babbdff79c4fa38904ff8d128'
}]

export default function accounts(state = initialAccounts, action) {
  switch (action.type) {
      case ADD_ACCOUNT:
        return [
          ...state,
          {
            address: action.address
          }
        ]

      default:
        return state
  }
}
