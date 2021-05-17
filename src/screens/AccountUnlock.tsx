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

import React, { useContext, useState } from 'react';
import { StyleSheet } from 'react-native';
import { CommonActions } from '@react-navigation/native';

import { AccountsContext } from 'stores/AccountsContext';
import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import { NetworksContext } from 'stores/NetworkContext';
import { ScannerContext } from 'stores/ScannerContext';
import { NavigationProps } from 'types/props';
import colors from 'styles/colors';
import fontStyles from 'styles/fontStyles';
import ScreenHeading from 'components/ScreenHeading';
import TextInput from 'components/TextInput';

/* Used for unlock and sign tx and messages for legacy accounts */
export function AccountUnlockAndSign(
	props: NavigationProps<'AccountUnlockAndSign'>
): React.ReactElement {
	const { navigation, route } = props;
	const next = route.params.next ?? 'SignedTx';
	const scannerStore = useContext(ScannerContext);
	const { getNetwork } = useContext(NetworksContext);

	return (
		<AccountUnlockView
			checkPin={async (pin: string): Promise<boolean> => {
				try {
					await scannerStore.signDataLegacy(pin, getNetwork);
					return true;
				} catch (e) {
					return false;
				}
			}}
			navigate={(): void => {
				const resetAction = CommonActions.reset({
					index: 1,
					routes: [
						{
							name: 'LegacyAccountList'
						},
						{ name: next }
					]
				});
				navigation.dispatch(resetAction);
			}}
		/>
	);
}

export function AccountUnlock({
	navigation,
	route
}: NavigationProps<'AccountUnlock'>): React.ReactElement {
	const next = route.params.next ?? 'LegacyAccountList';
	const onDelete = route.params.onDelete ?? ((): any => null);
	const accountsStore = useContext(AccountsContext);
	const { selectedKey } = accountsStore.state;

	return (
		<AccountUnlockView
			checkPin={async (pin: string): Promise<boolean> => {
				return await accountsStore.unlockAccount(selectedKey, pin);
			}}
			navigate={(): void => {
				if (next === 'AccountDelete') {
					navigation.goBack();
					onDelete();
				} else {
					const resetAction = CommonActions.reset({
						index: 2,
						routes: [
							{
								name: 'LegacyAccountList'
							},
							{ name: 'AccountDetails' },
							{ name: next }
						]
					});
					navigation.dispatch(resetAction);
				}
			}}
		/>
	);
}

interface AccountUnlockViewProps {
	checkPin: (pin: string) => Promise<boolean>;
	hasWrongPin?: boolean;
	navigate: () => void;
}

/*
interface AccountUnlockViewState {
	hasWrongPin: boolean;
	pin: string;
}
*/

function AccountUnlockView(props: AccountUnlockViewProps): React.ReactElement {
	const [hasWrongPin, setHasWrongPin] = useState(false);
	const [pin, setPin] = useState('');

	const showErrorMessage = (): string =>
		hasWrongPin ? 'Wrong pin, please try again' : '';

	const { checkPin, navigate } = props;

	return (
		<SafeAreaViewContainer style={styles.body}>
			<ScreenHeading
				title={'Unlock Account'}
				subtitle={showErrorMessage()}
				error={hasWrongPin}
			/>
			<PinInput
				label="PIN"
				onChangeText={async (inputPin: string): Promise<void> => {
					setPin(inputPin);
					if (inputPin.length < 4) {
						return;
					}
					if (await checkPin(inputPin)) {
						navigate();
					} else if (inputPin.length > 5) {
						setHasWrongPin(true);
					}
				}}
				value={pin}
			/>
		</SafeAreaViewContainer>
	);
}

function PinInput(props: any): React.ReactElement {
	return (
		<TextInput
			autoFocus
			keyboardAppearance="dark"
			clearTextOnFocus
			editable
			fontSize={24}
			keyboardType="numeric"
			multiline={false}
			autoCorrect={false}
			numberOfLines={1}
			returnKeyType="next"
			secureTextEntry
			style={[fontStyles.t_seed, styles.pinInput]}
			{...props}
		/>
	);
}

const styles = StyleSheet.create({
	body: {
		backgroundColor: colors.background.app,
		flex: 1,
		overflow: 'hidden'
	},
	pinInput: {
		borderColor: colors.border.light,
		minHeight: 48,
		paddingLeft: 10,
		paddingRight: 10
	}
});
