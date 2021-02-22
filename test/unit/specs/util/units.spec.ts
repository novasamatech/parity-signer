// Copyright 2015-2020 Parity Technologies (UK) Ltd.
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

import { Metadata } from '@polkadot/metadata';
import { TypeRegistry } from '@polkadot/types';
import { GenericCall as Call } from '@polkadot/types/generic';
import { formatBalance } from '@polkadot/util';

import { polkadotMetaData } from 'constants/networkMetadata';
import { fromWei } from 'utils/units';
const registry = new TypeRegistry();
registry.setMetadata(new Metadata(registry, polkadotMetaData));

describe('units', () => {
	describe('ethereum', () => {
		it('should properly convert units from wei', () => {
			const wei = '5208';
			const ether = fromWei(wei);
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

	describe('polkadot', () => {
		let method_1: Call;
		let method_2: Call;
		let method_3: Call;

		const getResultFromMethod = (method: Call): any => {
			const { args, meta } = method;

			const result = {} as any;
			for (let i = 0; i < meta.args.length; i++) {
				let value;
				if (
					args[i].toRawType() === 'Balance' ||
					args[i].toRawType() === 'Compact<Balance>'
				) {
					value = formatBalance(args[i].toString(), undefined, 12);
				} else {
					value = args[i].toString();
				}
				result[meta.args[i].name.toString()] = value;
			}
			return result;
		};

		beforeAll(() => {
			formatBalance.setDefaults({
				decimals: 10,
				unit: 'DOT'
			});

			method_1 = new Call(
				registry,
				'0x03000000002fac081000000000b48f6ee79343db082e913f105f7c6086cc1a60dbe4b9ff67ec9fe8c19526d471d8ed5886051198052a57ec34bcb5a458398bc3d93d7353f51cbd39f07000f80789c1e6b7a591bf461cf0b6e116dea135539578ecd3eb19764ae0f0b04cabac0e'
			);
			method_2 = new Call(
				registry,
				'0x03000000002fac081000000000b48f6ee79343db082e913f105f7c6086cc1a60dbe4b9ff67ec9fe8c19526d471d8ed5886051198052a57ec34bcb5a458398bc3d93d7353f51cbd39f07000f80789c1e6b7a591bf461cf0b6e116dea135539578ecd3eb19764ae0f0b04cabac0e'
			);
			method_3 = new Call(
				registry,
				'0x03000000002fac081000000000b48f6ee79343db082e913f105f7c6086cc1a60dbe4b9ff67ec9fe8c19526d471d8ed5886051198052a57ec34bcb5a458398bc3d93d7353f51cbd39f07000f80789c1e6b7a591bf461cf0b6e116dea135539578ecd3eb19764ae0f0b04cabac0e'
			);
		});

		it('should format KSM', () => {
			const result = getResultFromMethod(method_1);

			console.log(method_1)
			expect(result.dest).toBe(
				'1egYCubF1U5CGWiXjQnsXduiJYP49KTs8eX1jn1JrTqCYyQ'
			);
			expect(result.value).toBe('123.000 KSM');
		});

		it('should format decimals for less than one KSM', () => {
			const result = getResultFromMethod(method_2);

			expect(result.dest).toBe(
				'5GtKezSWWfXCNdnC4kkb3nRF9tn3NiN6ZWSEf7UaFdfMUanc'
			);
			expect(result.value).toBe('123.123µ KSM');
		});

		it('should format absurdly large KSM', () => {
			const result = getResultFromMethod(method_3);

			expect(result.dest).toBe(
				'5GzJiY3oG9LcyDiJbEJ6UF8jDF1AGeE2MgeXgSwgGCPopWsb'
			);
			expect(result.value).toBe('9.999T KSM');
		});
	});
});
