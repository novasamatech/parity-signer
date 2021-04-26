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

import { defaultPolkadotMetadata } from 'constants/networkMetadataList';
import { fromWei } from 'utils/units';
const registry = new TypeRegistry();
registry.setMetadata(new Metadata(registry, defaultPolkadotMetadata));

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
					value = formatBalance(args[i].toString(), undefined, 10);
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
				'0x0503008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480b008cb6611e01'
			);
			method_2 = new Call(
				registry,
				'0x0503008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48ce830700'
			);
			method_3 = new Call(
				registry,
				'0x0503008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a4833ffffffff9f36f400d946dad510ee8507'
			);
		});

		it('should format DOT', () => {
			const result = getResultFromMethod(method_1);

			expect(result.dest).toBe(
				//substrate address '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty'
				'14E5nqKAp3oAJcmzgZhUD2RcptBeUBScxKHgJKU4HPNcKVf3'
			);
			expect(result.value).toBe('123.0000 DOT');
		});

		it('should format decimals for less than one DOT', () => {
			const result = getResultFromMethod(method_2);

			expect(result.dest).toBe(
				//substrate address '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty'
				'14E5nqKAp3oAJcmzgZhUD2RcptBeUBScxKHgJKU4HPNcKVf3'
			);
			expect(result.value).toBe('12.3123 µDOT');
		});

		it('should format absurdly large KSM', () => {
			const result = getResultFromMethod(method_3);

			expect(result.dest).toBe(
				//substrate address '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty'
				'14E5nqKAp3oAJcmzgZhUD2RcptBeUBScxKHgJKU4HPNcKVf3'
			);
			expect(result.value).toBe('999.9999 YDOT');
		});
	});
});
