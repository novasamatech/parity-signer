// Copyright 2015-2021 Parity Technologies (UK) Ltd.
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

import React, { useState } from 'react';
import { Text, View } from 'react-native';

import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import { KeyboardAwareContainer } from 'modules/unlock/components/Container';
import PinInput from 'modules/unlock/components/PinInput';
import ScreenHeading from 'components/ScreenHeading';
import Button from 'components/Button';
import t from 'modules/unlock/strings';
import testIDs from 'e2e/testIDs';
import { NavigationProps } from 'types/props';
import QrView from 'components/QrView';
import fontStyles from 'styles/fontStyles';
import styles from 'modules/sign/styles';
import Separator from 'components/Separator';
import { sign } from 'utils/native';

function SignedTx({
	route,
	_navigation
}: NavigationProps<'SignedTx'>): React.ReactElement {
	const [signedData, setSignedData] = useState<string>(''); //route.params.action.payload;
	const [pin, setPin] = useState<string>('');
	const [password, setPassword] = useState<string>('');
	const [focusPassword, setFocusPassword] = useState<boolean>(false);
	const [buttonDisabled, setButtonDisabled] = useState(false);
	const [errorMessage, setErrorMessage] = useState<string>('');

	async function submit(): Promise<void> {
		setButtonDisabled(true);
		setFocusPassword(false);
		if (pin.length >= 6) {
			try {
				console.log(pin);
				console.log(typeof pin);
				console.log(password);
				console.log(typeof password);
				const toSign = route.params ? JSON.stringify(route.params.payload) : '';
				console.log(toSign);
				console.log(typeof toSign);
				const signerOutput = await sign(
					toSign,
					pin.toString(),
					password.toString()
				);
				setSignedData(signerOutput);
			} catch (e) {
				console.log(e);
				setErrorMessage(e.toString());
				//TODO record error times;
			}
		} //TODO else { setAlert('pin too short (at least 6 numbers'); }
	}

	//IMPORTANT: nothing but QR code and address name should be shown here; showing address path is dangerous
	if (signedData) {
		return (
			<SafeAreaScrollViewContainer>
				<Text style={styles.topTitle}>Signed extrinsic</Text>
				<Separator
					shadow={true}
					style={{
						height: 0,
						marginVertical: 20
					}}
				/>
				<Text style={[fontStyles.h_subheading, { paddingHorizontal: 16 }]}>
					{'Scan to publish'}
				</Text>
				<View style={styles.qr} testID={testIDs.SignedTx.qrView}>
					<QrView data={signedData} />
				</View>
			</SafeAreaScrollViewContainer>
		);
	} else {
		return (
			<KeyboardAwareContainer
				contentContainerStyle={{
					flexGrow: 1
				}}
			>
				<ScreenHeading
					title={t.title.pinUnlock}
					error={!!errorMessage}
					subtitle={errorMessage}
				/>
				<PinInput
					label={t.pinLabel}
					autoFocus
					testID={testIDs.IdentityPin.unlockPinInput}
					onChangeText={(newInput: string): void => {
						setButtonDisabled(false);
						setPin(newInput);
					}}
					onSubmitEditing={(): void => setFocusPassword(true)}
					value={pin}
				/>
				<PinInput
					label={t.passwordLabel}
					testID={testIDs.IdentityPin.passwordInput}
					returnKeyType="done"
					keyboardType="default"
					focus={focusPassword}
					onChangeText={(newInput: string): void => {
						setButtonDisabled(false);
						setPassword(newInput);
					}}
					onSubmitEditing={submit}
					value={password}
				/>
				<Button
					disabled={buttonDisabled}
					title={t.doneButton.pinUnlock}
					onPress={submit}
					testID={testIDs.IdentityPin.unlockPinButton}
				/>
			</KeyboardAwareContainer>
		);
	}
}

export default SignedTx;
