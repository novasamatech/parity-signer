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

import { hexToU8a } from '@polkadot/util';

import kusamaMeta from './static-kusama';
import { base64ToHex, checkIfPayloadIsMetadata } from './strings';

const BASE64 =
	'bWV0YQhgGFN5c3RlbQABHChmaWxsX2Jsb2NrAAQhASBBIGJpZyBkaXNwYXRjaCB0aGF0IHdpbGwgZGlzYW';

describe('strings', () => {
	it('convert base64 to hex works', () => {
		const hex = base64ToHex(BASE64);

		expect(hex).toBeDefined();
		expect(hex).toBe(
			'0x6D65746108601853797374656D00011C2866696C6C5F626C6F636B0004210120412062696720646973706174636820746861742077696C6C2064697361'
		);
	});

	it('checks if payload is proper metadata', () => {
		const payload = hexToU8a(base64ToHex(kusamaMeta));

		const checkResult = checkIfPayloadIsMetadata(payload);

		expect(checkResult).toBe(true);
	});

	it('errors if payload is not metadata', () => {
		const payload = BASE64;

		const checkResult = checkIfPayloadIsMetadata(payload);

		expect(checkResult).toBe(false);
	});
});
