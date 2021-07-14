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

import React, { useEffect, useState } from 'react';
import { Text, View, ActivityIndicator, StyleSheet } from 'react-native';

import { SafeAreaScrollViewContainer, SafeAreaViewContainer } from 'components/SafeAreaContainer';
import { KeyboardAwareContainer } from 'components/Container';
import PinInput from 'components/PinInput';
import ScreenHeading from 'components/ScreenHeading';
import Button from 'components/Button';
import testIDs from 'e2e/testIDs';
import { NavigationProps } from 'types/props';
import QrView from 'components/QrView';
import fontStyles from 'styles/fontStyles';
import Separator from 'components/Separator';
import { sign } from 'utils/native';

function SignedTx({
	route,
	navigation
}: NavigationProps<'SignedTx'>): React.ReactElement {
	const [signedData, setSignedData] = useState<string>(''); //route.params.action.payload;
	const [password, setPassword] = useState<string>('');
	const [buttonDisabled, setButtonDisabled] = useState(false);
	const [errorMessage, setErrorMessage] = useState<string>('');
	const payload = route.params.payload;
	const author = route.params.author;
	const toSign = route.params ? JSON.stringify(route.params.payload) : '';

	useEffect(() => {
		console.log(author);
		const generateSignedData = async (): void => {
			console.log(author);
			console.log(typeof author.seed);
			console.log(toSign);
			console.log(typeof toSign);
			try {
				const signerOutput = await sign(
					toSign,
					author.seed,
					""
				);
				setSignedData(signerOutput);
			} catch (e) {
				console.log(e);
				setErrorMessage(e.toString());
			}
		}
		if (author.has_password === 'false') {
			generateSignedData();
		}
	}, []);

	async function submit(): Promise<void> {
		setButtonDisabled(true);
		try {
			console.log(password);
			console.log(typeof password);
			console.log(author);
			console.log(typeof author.seed);
			console.log(toSign);
			console.log(typeof toSign);
			const signerOutput = await sign(
				toSign,
				author.seed,
				password.toString()
			);
			setSignedData(signerOutput);
		} catch (e) {
			console.log(e);
			setErrorMessage(e.toString());
			//TODO record error times;
		}
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
				<Text style={[fontStyles.h_subheading, { paddingHorizontal: 16 }]}>
					{"Signed by: " + author.name + "; for seed: " + author.seed}
				</Text>
			</SafeAreaScrollViewContainer>
		);
	} else if (author.has_password === 'true') {
		return (
			<KeyboardAwareContainer
				contentContainerStyle={{
					flexGrow: 1
				}}
			>
				<ScreenHeading
					title="Please enter password"
					error={!!errorMessage}
					subtitle={errorMessage}
				/>
				<PinInput
					label={t.passwordLabel}
					autoFocus
					testID={testIDs.IdentityPin.passwordInput}
					returnKeyType="done"
					keyboardType="default"
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
	} else {
		return (
			<SafeAreaViewContainer>
				<ScreenHeading
					title="Signed message not ready"
					error={!!errorMessage}
					subtitle={errorMessage}
				/>
				<ActivityIndicator
					animating={true}
					color="red"
					size="large"
					style={{margin:15}}
				/>
			</SafeAreaViewContainer>
		);
	}
}

export default SignedTx;

const styles = StyleSheet.create({
	body: {
		paddingTop: 24
	},
	bodyContent: {
		marginVertical: 16,
		paddingHorizontal: 20
	},
	qr: {
		marginBottom: 8
	},
	title: {
		...fontStyles.h2,
		paddingBottom: 20
	},
	topTitle: {
		...fontStyles.h1,
		textAlign: 'center'
	}
});

