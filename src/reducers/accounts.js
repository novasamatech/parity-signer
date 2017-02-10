import { ADD_ACCOUNT } from '../constants/AccountActions'

const initialAccounts = []

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
