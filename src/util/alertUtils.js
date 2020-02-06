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

import { Alert, Clipboard } from 'react-native';

export const alertErrorWithMessage = (message, buttonText) =>
	Alert.alert('Error', message, [
		{
			style: 'Cancel',
			text: buttonText
		}
	]);

export const alertIdentityCreationError = () =>
	alertErrorWithMessage("Can't create Identity from the seed", 'Try again');

export const alertPathDerivationError = () =>
	alertErrorWithMessage("Can't Derive Key pairs from the seed", 'Try again');

export const alertPathDeletionError = () =>
	alertErrorWithMessage("Can't delete Key pairs.", 'Try again');

export const alertIdentityDeletionError = () =>
	alertErrorWithMessage("Can't delete Identity.", 'Try again');

const buildAlertButtons = (onConfirm, confirmText) => [
	{
		onPress: () => {
			onConfirm();
		},
		style: 'destructive',
		text: confirmText
	},
	{
		style: 'cancel',
		text: 'Cancel'
	}
];

const buildAlertDeleteButtons = onDelete =>
	buildAlertButtons(onDelete, 'Delete');

export const alertDeleteAccount = (accountName, onDelete) => {
	Alert.alert(
		'Delete Key Pairs',
		`Do you really want to delete ${accountName}?`,
		buildAlertDeleteButtons(onDelete)
	);
};

export const alertDeleteLegacyAccount = (accountName, onDelete) => {
	Alert.alert(
		'Delete Key Pairs',
		`Do you really want to delete ${accountName}?
The account can only be recovered with its associated recovery phrase.`,
		buildAlertDeleteButtons(onDelete)
	);
};

export const alertDeleteIdentity = onDelete => {
	Alert.alert(
		'Delete Identity',
		`Do you really want to delete this Identity and all the related accounts?
This identity can only be recovered with its associated recovery phrase.`,
		buildAlertDeleteButtons(onDelete)
	);
};

export const alertCopyBackupPhrase = seedPhrase =>
	Alert.alert(
		'Write this recovery phrase on paper',
		'It is not recommended to transfer or store a recovery phrase digitally and unencrypted. Anyone in possession of this recovery phrase is able to spend funds from this account.',
		[
			{
				onPress: () => {
					Clipboard.setString(seedPhrase);
				},
				style: 'default',
				text: 'Copy anyway'
			},
			{
				style: 'cancel',
				text: 'Cancel'
			}
		]
	);

export const alertRisks = (message, onPress) =>
	Alert.alert('Warning', message, [
		{
			onPress,
			style: 'default',
			text: 'I understand the risks'
		},
		{
			style: 'cancel',
			text: 'Back'
		}
	]);

export const alertMultipart = onNext =>
	alertRisks(
		'The payload of the transaction you are signing is too big to be decoded. Not seeing what you are signing is inherently unsafe. If possible, contact the developer of the application generating the transaction to ask for multipart support.',
		onNext
	);

export const alertDecodeError = () =>
	Alert.alert(
		'Could not decode method with available metadata.',
		'Signing something you do not understand is inherently unsafe. Do not sign this extrinsic unless you know what you are doing, or update Parity Signer to be able to decode this message. If you are not sure, or you are using the latest version, please open an issue on github.com/paritytech/parity-signer.',
		[
			{
				style: 'default',
				text: 'Okay'
			}
		]
	);

export const alertBackupDone = onPress =>
	Alert.alert(
		'Important',
		"Make sure you've backed up this recovery phrase. It is the only way to restore your account in case of device failure/lost.",
		[
			{
				onPress,
				text: 'Proceed'
			},
			{
				style: 'cancel',
				text: 'Cancel'
			}
		]
	);
