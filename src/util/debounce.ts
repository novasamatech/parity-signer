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
 * Creates and returns a new debounced version of the passed function that will
 * postpone its execution until after wait milliseconds have elapsed since
 * the last time it was invoked.
 *
 * @type  {T}                item    type
 * @param {(any) => any}     function to debounce
 * @param {number}           time in milliseconds
 *
 *
 * @return {any}            the debounced function
 */
let timeout: any;

export function debounce(fn: any, time: number): () => void {
	return function debouncedFunction(...args): void {
		const functionCall = (): any => fn.apply(null, ...args);

		clearTimeout(timeout);
		timeout = setTimeout(functionCall, time);
	};
}
