// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// Copyright 2021 Commonwealth Labs, Inc.
// This file is part of Layer Wallet.

// Layer Wallet is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Layer Wallet is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Layer Wallet. If not, see <http://www.gnu.org/licenses/>.

import testIDs from 'e2e/testIDs';
import { Action, SetAlert } from 'stores/alertContext';

const alertTestIDs = testIDs.Alert;
export const alertError = (setAlert: SetAlert, message: string): void =>
	setAlert('Error', message);

export const alertIdentityCreationError = (
	setAlert: SetAlert,
	errorMessage: string
): void =>
	alertError(setAlert, "Can't create Identity from the seed: " + errorMessage);

export const alertPathDerivationError = (
	setAlert: SetAlert,
	errorMessage: string
): void =>
	alertError(setAlert, "Can't derive account from the seed: " + errorMessage);

const buildAlertButtons = (
	onConfirm: () => any,
	confirmText: string,
	testID?: string
): Action[] => [
	{
		onPress: (): void => {
			onConfirm();
		},
		testID,
		text: confirmText
	},
	{
		text: 'Cancel'
	}
];

const buildAlertDeleteButtons = (
	onDelete: () => any,
	testID?: string
): Action[] => buildAlertButtons(onDelete, 'Delete', testID);

export const alertDeleteAccount = (
	setAlert: SetAlert,
	accountName: string,
	onDelete: () => any
): void => {
	setAlert(
		'Delete Account',
		`Do you really want to delete ${accountName}?`,
		buildAlertDeleteButtons(onDelete, alertTestIDs.deleteAccount)
	);
};

export const alertDeleteIdentity = (
	setAlert: SetAlert,
	onDelete: () => any
): void => {
	setAlert(
		'Delete Identity',
		`Do you really want to delete this wallet? It can only be recovered with its secret phrase.`,
		buildAlertDeleteButtons(onDelete, alertTestIDs.deleteIdentity)
	);
};

export const alertRisks = (
	setAlert: SetAlert,
	message: string,
	onPress: () => any
): void =>
	setAlert('Warning', message, [
		{
			onPress,
			text: 'Proceed'
		},
		{
			text: 'Back'
		}
	]);

export const alertDecodeError = (setAlert: SetAlert): void =>
	setAlert(
		'Could not decode method with available metadata.',
		'Signing something you do not understand is inherently unsafe. Do not sign this extrinsic unless you know what you are doing, or update Parity Signer to be able to decode this message. If you are not sure, or you are using the latest version, please open an issue on github.com/paritytech/parity-signer.'
	);
