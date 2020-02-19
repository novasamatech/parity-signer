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

/**
 * Default comparator, should work for strings and numbers
 */
function defaultCompare(a: any, b: any): number {
	if (a > b) {
		return 1;
	}

	if (a < b) {
		return -1;
	}

	return 0;
}

/**
 * Find an index of an element within a sorted array. This should be substantially
 * faster than `indexOf` for large arrays.
 *
 * @type  {T}                item    type
 * @param {T}                item    to find
 * @param {Array<T>}         array   to look through
 * @param {(a, b) => number} [compare = defaultCompare] comparator function
 *
 * @return {{ hit: bool, index: number }} if `hit` is `true` -> index at which the item was found
 *                                        if `hit` is `false` -> index at which the item can be inserted
 */
export function binarySearch(
	array: Array<any>,
	item: any,
	compare: (a: any, b: any) => number = defaultCompare
): { hit: boolean; index: number } {
	if (array.length === 0) {
		return { hit: false, index: 0 };
	}

	let min = 0;
	let max = array.length - 1;

	while (min <= max) {
		const guess = (min + max) >> 1; // fast integer division by 2

		const result = compare(item, array[guess]);

		if (result < 0) {
			max = guess - 1;
		} else if (result > 0) {
			min = guess + 1;
		} else {
			return { hit: true, index: guess };
		}
	}

	return { hit: false, index: min };
}

export function zip(left: any[], right: any[]): any[] {
	let lindex = 0;
	let rindex = 0;
	let oindex = 0;

	// allocate enough memory to merge two arrays
	const out = new Array(left.length + right.length);

	while (lindex < left.length && rindex < right.length) {
		const lword = left[lindex];
		const rword = right[rindex];

		if (lword < rword) {
			out[oindex] = lword;
			lindex += 1;
		} else if (lword > rword) {
			out[oindex] = rword;
			rindex += 1;
		} else {
			out[oindex] = lword;
			lindex += 1;
			rindex += 1;
		}

		oindex += 1;
	}

	return out.slice(0, oindex);
}
