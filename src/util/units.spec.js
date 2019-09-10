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

'use strict';

import '@polkadot/types/injector';

import extrinsicsFromMeta from '@polkadot/api-metadata/extrinsics/fromMetadata';
import { Balance, Compact, createType, GenericCall, Metadata } from '@polkadot/types';
import Call from '@polkadot/types/primitive/Generic/Call';
import kusamaData from './static-kusama';
import { formatDecimal, fromWei } from './units';

describe('units', () => {
  describe('ethereum', () => {
    it('should properly convert units from wei', () => {
      let wei = '5208';
      let ether = fromWei(wei);
      expect(ether).toEqual('0.000000000000021');
    });
  
    it('should return BigNumber for undefined values', () => {
      expect(fromWei(null)).toEqual('0');
      expect(fromWei(undefined)).toEqual('0');
      expect(fromWei(0)).toEqual('0');
      expect(fromWei('0')).toEqual('0');
      expect(fromWei('')).toEqual('0');
    });
  });

  describe('kusama', () => {
    beforeAll(() => {
      const metadata = new Metadata(kusamaData);
    
      const extrinsics = extrinsicsFromMeta(metadata);

      GenericCall.injectMethods(extrinsics);
    });

    it('should properly format from Balance', () => {
      let hugeBalance = createType('Balance', 1234567898771);
      let formattedHuge = formatDecimal(hugeBalance.toString());

      expect(hugeBalance).toBeDefined();
      expect(hugeBalance.toString()).toBe('1234567898771');
      expect(formattedHuge).toBeDefined();
      expect(formattedHuge).toBe('1 234 567 898 771');
    });

    it('should work', () => {
      let method = new Call('0x0400ffd541fa133def7268cc0e5213aebf10ec04b822d59fb7556341f4e49911fc110a0b00b04e2bde6f');
      const { args, meta } = method;

      let result = {};
      for (let i = 0; i < meta.args.length; i ++) {
        let value;
        if (args[i].toRawType() === 'Balance' || args[i].toRawType() == 'Compact<Balance>') {
          value = formatDecimal(args[i].toString());
        } else {
          value = args[i].toString();
        }
        result[meta.args[i].name.toString()] = value;
      }

      expect(result.dest).toBe('5GtKezSWWfXCNdnC4kkb3nRF9tn3NiN6ZWSEf7UaFdfMUanc');
      expect(result.value).toBe('123 000 000 000 000');
    })
  });
});
