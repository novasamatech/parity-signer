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

import { ENABLE_SCANNER, DISABLE_SCANNER, DISABLE_SCANNER_WARNINGS, RESET_SCANNER } from '../constants/ScannerActions'

const initialState = {
  scannerEnabled: true,
  shouldDisplayWarning: true
}

export default function scanner (state = initialState, action) {
  switch (action.type) {
    case ENABLE_SCANNER:
      return {
        ...state,
        scannerEnabled: true
      }

    case DISABLE_SCANNER:
      return {
        ...state,
        scannerEnabled: false
      }

    case DISABLE_SCANNER_WARNINGS:
      return {
        ...state,
        shouldDisplayWarning: false
      }

    case RESET_SCANNER:
      return {
        ...state,
        scannerEnabled: true,
        shouldDisplayWarning: true
      }

    default:
      return state
  }
}
