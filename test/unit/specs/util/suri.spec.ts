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

import { constructSURI, parseDerivationPath, parseSURI } from 'utils/suri';

describe('derivation path', () => {
	describe('parsing', () => {
		it('should properly parse and return a derivation path object from a string containing soft, hard and password', () => {
			const parsedDerivationPath = parseDerivationPath(
				'/soft/soft//hard///mypassword'
			);

			expect(parsedDerivationPath).toBeDefined();
			expect(parsedDerivationPath.derivePath).toBe('/soft/soft//hard');
			expect(parsedDerivationPath.password).toBe('mypassword');
		});

		it('should throw if the string is not a valid derivation path', () => {
			const malformed = 'wrong/bla';

			expect(() => parseDerivationPath(malformed)).toThrowError(
				'Invalid derivation path input.'
			);
		});

		it('should accept a password alone', () => {
			const passwordAlone = '///mypassword';
			const parsedDerivationPath = parseDerivationPath(passwordAlone);

			expect(parsedDerivationPath).toBeDefined();
			expect(parsedDerivationPath.password).toBe('mypassword');
		});
	});
});

describe('suri', () => {
	describe('parsing', () => {
		it('should properly parse and return an SURI object from a string', () => {
			const suri = parseSURI(
				'six nine great ball dog over moon light//hard/soft///mypassword'
			);

			expect(suri).toBeDefined();
			expect(suri.phrase).toBe('six nine great ball dog over moon light');
			expect(suri.derivePath).toBe('//hard/soft');
			expect(suri.password).toBe('mypassword');
		});

		it('should throw if the string is not a valid suri', () => {
			const malformed = '1!,#(&(/)!_c.';

			expect(() => parseSURI(malformed)).toThrowError('Invalid SURI input.');
		});

		it('should throw if phrase was empty', () => {
			const missingPhrase = '//hard/soft///password';

			expect(() => parseSURI(missingPhrase)).toThrowError(
				'SURI must contain a phrase.'
			);
		});

		it('should parse string with extra spaces as brain wallet', () => {
			const extraSpaces = '  great  sparta  ';
			expect(parseSURI(extraSpaces)).toEqual({
				derivePath: '',
				password: '',
				phrase: '  great  sparta  '
			});

			const extraSpacesWithPath = '  great  sparta  //hard/soft///password';
			expect(parseSURI(extraSpacesWithPath)).toEqual({
				derivePath: '//hard/soft',
				password: 'password',
				phrase: '  great  sparta  '
			});
		});
	});

	describe('constructing', () => {
		it('should properly construct SURI string from object', () => {
			const suriObject = {
				derivePath: '//hard/soft',
				password: 'mypassword',
				phrase: 'six nine great ball dog over moon light'
			};

			const suri = constructSURI(suriObject);

			expect(suri).toBeDefined();
			expect(suri).toBe(
				'six nine great ball dog over moon light//hard/soft///mypassword'
			);
		});

		it('should throw if the suri object is not valid', () => {
			const empty = {};

			const malformed = {
				phrase: null
			};

			expect(() => constructSURI(empty as any)).toThrow(
				'Cannot construct an SURI from emtpy phrase.'
			);
			expect(() => constructSURI(malformed as any)).toThrow(
				'Cannot construct an SURI from emtpy phrase.'
			);
		});
	});
});
