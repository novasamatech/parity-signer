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

'use strict'

import { NEW_SCANNED_TX, NEW_SCANNED_DATA, SIGN_HASH } from '../constants/SignerActions'

const initialState = {
  transactionDetails: {},
  data: '',
  hashToSign: '',
  signedHash: ''
}

export default function signer (state = initialState, action) {
  switch (action.type) {
    case NEW_SCANNED_TX:
      return Object.assign({}, state, {
        transactionDetails: action.transaction,
        hashToSign: action.hash
      })
    case NEW_SCANNED_DATA:
      return Object.assign({}, state, {
        data: action.data,
        hashToSign: action.hash
      })
    case SIGN_HASH:
      return Object.assign({}, state, {
        signedHash: action.signature
      })
    default:
      return state
  }
}
