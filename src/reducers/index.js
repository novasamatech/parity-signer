'use strict'

import { combineReducers } from 'redux'
import accounts from './accounts'
import routes from './routes'
import transactions from './transactions'

export default combineReducers({
  accounts,
  routes,
  transactions
})
