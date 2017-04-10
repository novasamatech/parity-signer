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

import { checksummedAddress } from './checksum'

describe('checksum', () => {
  it('create a proper checksum of an address', () => {
    let address = '006e27b6a72e1f34c626762f3c4761547aff1421'
    let hash = '7c9802b5bbbec094f42a2e0bdb4c1b1cbf54e334fd7c7be5cad65936fa0e3d74'
    let result = checksummedAddress(address, hash)
    expect(result).toEqual('006E27B6A72E1f34C626762F3C4761547Aff1421')
  })

  it('create a proper checksum of an address 2', () => {
    let address = '007a3fa1d163f66eed8f8dddb9346610d603c7a1'
    let hash = '666f32afc164ae61524198c5f336aefdcfba967990a5a948a22aba96f41b5286'
    let result = checksummedAddress(address, hash)
    expect(result).toEqual('007A3fA1D163F66eed8f8DDdB9346610D603C7a1')
  })
})
