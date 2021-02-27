// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// Modifications Copyright (c) 2021 Thibaut Sardan

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

const t = {
	doneButton: {
		nameChange: 'Done',
		pinCreation: 'Done',
		pinUnlock: 'Unlock'
	},
	passwordLabel: 'Password',
	pinConfirmLabel: 'Confirm PIN',
	pinLabel: 'PIN',
	pinMisMatchHint: {
		pinCreation: "Pin codes don't match!",
		pinUnlock: 'Input credential is not correct!'
	},
	pinTooShortHint: 'Your pin must be at least 6 digits long!',
	subtitle: {
		pinCreation: 'Choose a PIN code with 6 or more digits',
		pinUnlock: 'Unlock the identity to use the seed'
	},
	title: {
		pinCreation: 'Set Identity PIN',
		pinUnlock: 'Unlock Identity'
	}
};

export default t;
