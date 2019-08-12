// Copyright 2015-2019 Parity Technologies (UK) Ltd.
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

// @flow

import React from 'react';
import TestRenderer from 'react-test-renderer';
import { Provider, Subscribe, Container } from 'unstated';

import AccountsStore from './AccountsStore';
import ScannerStore from './ScannerStore';

function render(element) {
  return TestRenderer.create(element).toJSON();
}

describe('QrScanner', () => {
  it('should do shit properly', () => {
    let accountsStore = new AccountsStore();
    let scannerStore = new ScannerStore();
    let tree = render(
      <Provider inject={[accountsStore, scannerStore]}>
        <Subscribe to={[ScannerStore, AccountsStore]}>
          {async (scannerStore, accountsStore) => {
              if (scannerStore.isBusy()) {
                return;
              }

              if (txRequestData.data) { // Ethereum Legacy
                await scannerStore.setUnsigned(txRequestData.data);
              } else {
                try {
                  await scannerStore.setParsedData(txRequestData.rawData, accountsStore);
                } catch (e) {
                  // Alert
                  console.error(e);
                }
              }

              if (!(await scannerStore.setData(accountsStore))) {
                return;
              } else {
                if (scannerStore.getType() === 'transaction') {
                  console.log('Navigating to TxDetails');
                } else {
                  console.log('Navigating to MessageDetails');
                }
              }
            }}
        </Subscribe>
      </Provider>
    );

    expect(scannerStore).toBeDefined();
  })
});
