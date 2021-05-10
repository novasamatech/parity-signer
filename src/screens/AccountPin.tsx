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

//Deprecated
import React, { useContext, useReducer } from 'react';
import { StyleSheet, Text } from 'react-native';
import { CommonActions } from '@react-navigation/native';

import { AccountsContext } from 'stores/AccountsContext';
import { NetworksContext } from 'stores/NetworkContext';
import fontStyles from 'styles/fontStyles';
import { NavigationProps } from 'types/props';
import colors from 'styles/colors';
import Button from 'components/Button';
import KeyboardScrollView from 'components/KeyboardScrollView';
import TextInput from 'components/TextInput';
import fonts from 'styles/fonts';
import { navigateToLegacyAccountList } from 'utils/navigationHelpers';

interface State {
	confirmation: string;
	focusConfirmation: boolean;
	pin: string;
	pinMismatch: boolean;
	pinTooShort: boolean;
}
function AccountPin({
	navigation,
	route
}: NavigationProps<'AccountPin'>): React.ReactElement {
	const accountsStore = useContext(AccountsContext);
	const { allNetworks } = useContext(NetworksContext);
	const initialState: State = {
		confirmation: '',
		focusConfirmation: false,
		pin: '',
		pinMismatch: false,
		pinTooShort: false
	};

	const reducer = (state: State, delta: Partial<State>): State => ({
		...state,
		...delta
	});
	const [state, setState] = useReducer(reducer, initialState);

	const submit = async (): Promise<void> => {
		const { selectedKey, newAccount } = accountsStore.state;
		const { pin, confirmation } = state;
		const accountCreation: boolean = route.params?.isNew ?? false;
		const account = accountCreation ? newAccount : accountsStore.getSelected()!;
		if (pin.length >= 6 && pin === confirmation) {
			if (accountCreation) {
				await accountsStore.submitNew(pin, allNetworks);
				return navigateToLegacyAccountList(navigation);
			} else {
				await accountsStore.save(selectedKey, account, pin);
				const resetAction = CommonActions.reset({
					index: 1,
					routes: [{ name: 'LegacyAccountList' }, { name: 'AccountDetails' }]
				});
				navigation.dispatch(resetAction);
			}
		} else {
			if (pin.length < 6) {
				setState({ pinTooShort: true });
			} else if (pin !== confirmation) setState({ pinMismatch: true });
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

	const title = 'ACCOUNT PIN';
	return (
		<KeyboardScrollView style={styles.body} extraHeight={120}>
			<Text style={styles.titleTop}>{title}</Text>
			{showHintOrError()}
			<Text style={styles.title}>PIN</Text>
			<PinInput
				autoFocus
				returnKeyType="next"
				onFocus={(): void => setState({ focusConfirmation: false })}
				onSubmitEditing={(): void => {
					setState({ focusConfirmation: true });
				}}
				onChangeText={(pin: string): void => onPinInputChange('pin', pin)}
				value={state.pin}
			/>
			<Text style={styles.title}>CONFIRM PIN</Text>
			<PinInput
				returnKeyType="done"
				focus={state.focusConfirmation}
				onChangeText={(confirmation: string): void =>
					onPinInputChange('confirmation', confirmation)
				}
				value={state.confirmation}
			/>
			<Button onPress={submit} title="Done" />
		</KeyboardScrollView>
	);
}

function PinInput(props: any): React.ReactElement {
	return (
		<TextInput
			keyboardAppearance="dark"
			clearTextOnFocus
			editable
			keyboardType="numeric"
			multiline={false}
			autoCorrect={false}
			numberOfLines={1}
			returnKeyType="next"
			secureTextEntry
			style={StyleSheet.flatten([styles.pinInput, { fontSize: 24 }])}
			{...props}
		/>
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
