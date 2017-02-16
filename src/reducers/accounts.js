import { ADD_ACCOUNT } from '../constants/AccountActions'

const initialAccounts = [{address: 'dupa'}, {address: 'dupa2'}]

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
