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

import { CommonActions } from '@react-navigation/native';
import Button from 'components/Button';
import KeyboardScrollView from 'components/KeyboardScrollView';
import TextInput from 'components/TextInput';
import React, { useContext, useReducer } from 'react';
import { StyleSheet, Text } from 'react-native';
import colors from 'styles/colors';
import fonts from 'styles/fonts';
import fontStyles from 'styles/fontStyles';
import { NavigationProps } from 'types/props';
import { navigateToLegacyAccountList } from 'utils/navigationHelpers';

import { AccountsContext } from '../context';

interface State {
	confirmation: string;
	focusConfirmation: boolean;
	pin: string;
	pinMismatch: boolean;
	pinTooShort: boolean;
}

function PinInput(props: any): React.ReactElement {
	return (
		<TextInput
			autoCorrect={false}
			clearTextOnFocus
			editable
			keyboardAppearance="dark"
			keyboardType="numeric"
			multiline={false}
			numberOfLines={1}
			returnKeyType="next"
			secureTextEntry
			style={StyleSheet.flatten([styles.pinInput, { fontSize: 24 }])}
			{...props}
		/>
	);
}

const initialState: State = {
	confirmation: '',
	focusConfirmation: false,
	pin: '',
	pinMismatch: false,
	pinTooShort: false
};

function AccountPin({ navigation, route }: NavigationProps<'AccountPin'>): React.ReactElement {
	const accountsStore = useContext(AccountsContext);

	const reducer = (state: State, delta: Partial<State>): State => ({
		...state,
		...delta
	});
	const [state, setState] = useReducer(reducer, initialState);

	const submit = async (): Promise<void> => {
		const { getSelectedAccount, lockAccount, newAccount } = accountsStore;
		const selectedAccount = getSelectedAccount();
		const { confirmation, pin } = state;
		const accountCreation = route.params?.isNew;
		const account = accountCreation ? newAccount : selectedAccount;

		if (pin.length < 6) {
			setState({ pinTooShort: true });

			return
		}

		if (pin !== confirmation) {
			setState({ pinMismatch: true });

			return
		}

		if (!account) {
			console.error('No account provided');

			return
		}

		if (accountCreation) {
			// this is the first time a pin is created
			await accountsStore.submitNew(pin);

			return navigateToLegacyAccountList(navigation);
		} else {
			// this is a pin change
			await accountsStore.saveAccount(account, pin);
			lockAccount(account.address);
			const resetAction = CommonActions.reset({
				index: 1,
				routes: [{ name: 'LegacyAccountList' }, { name: 'AccountDetails' }]
			});

			navigation.dispatch(resetAction);
		}
	};

	const showHintOrError = (): React.ReactElement => {
		if (state.pinTooShort) {
			return (
				<Text style={styles.errorText}>
					Your pin must be at least 6 digits long!
				</Text>
			);
		} else if (state.pinMismatch) {
			return <Text style={styles.errorText}>Pin codes don't match!</Text>;
		}

		return (
			<Text style={styles.hintText}>
				Choose a PIN code with 6 or more digits
			</Text>
		);
	};

	const onPinInputChange = (stateName: string, pinInput: string): void => {
		if (/^\d+$|^$/.test(pinInput)) {
			setState({
				pinMismatch: false,
				pinTooShort: false,
				[stateName]: pinInput
			});
		}
	};

	return (
		<KeyboardScrollView
			extraHeight={120}
			style={styles.body}
		>
			<Text style={styles.titleTop}>{'ACCOUNT PIN'}</Text>
			{showHintOrError()}
			<Text style={styles.title}>PIN</Text>
			<PinInput
				autoFocus
				onChangeText={(pin: string): void => onPinInputChange('pin', pin)}
				onFocus={(): void => setState({ focusConfirmation: false })}
				onSubmitEditing={(): void => { setState({ focusConfirmation: true }); }}
				returnKeyType="next"
				value={state.pin}
			/>
			<Text style={styles.title}>CONFIRM PIN</Text>
			<PinInput
				focus={state.focusConfirmation}
				onChangeText={(confirmation: string): void => onPinInputChange('confirmation', confirmation) }
				returnKeyType="done"
				value={state.confirmation}
			/>
			<Button
				onPress={submit}
				title="Done"
			/>
		</KeyboardScrollView>
	);
}

export default AccountPin;

const styles = StyleSheet.create({
	body: {
		padding: 20
	},
	errorText: {
		color: colors.signal.error,
		fontFamily: fonts.bold,
		fontSize: 12,
		paddingBottom: 20,
		textAlign: 'center'
	},
	hintText: {
		color: colors.text.faded,
		fontFamily: fonts.bold,
		fontSize: 12,
		paddingBottom: 20,
		textAlign: 'center'
	},
	pinInput: {
		marginBottom: 20
	},
	title: {
		...fontStyles.h_subheading,
		color: colors.text.main
	},
	titleTop: {
		color: colors.text.main,
		fontFamily: fonts.bold,
		fontSize: 24,
		paddingBottom: 20,
		textAlign: 'center'
	}
});
