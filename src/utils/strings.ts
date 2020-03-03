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

/*
 * @dev Check if input is in Ascii table.
 */
export function isAscii(data: string | number): boolean {
	/* eslint-disable-next-line no-control-regex */
	return /^[\x00-\x7F]*$/.test(data as string);
}

/*
 * @dev Take hex encoded binary and make it utf-8 readable
 */
export function hexToAscii(hexx: string): string {
	const hex = hexx.toString();
	let str = '';
	for (let i = 0; i < hex.length; i += 2) {
		str += String.fromCharCode(parseInt(hex.substr(i, 2), 16));
	}

	return str;
}

/*
 * @dev Take a long string and output the first and last 10 chars.
 */

export function shortString(original: string): string {
	return original
		.substr(0, 20)
		.concat('......')
		.concat(original.substr(original.length - 20));
}
