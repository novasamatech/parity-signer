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

//Deprecated

import React, { useReducer } from 'react';
import { StyleSheet, Text } from 'react-native';
import { NavigationActions, StackActions } from 'react-navigation';

import { NavigationAccountProps } from 'types/props';
import colors from 'styles/colors';
import Background from 'components/Background';
import Button from 'components/Button';
import KeyboardScrollView from 'components/KeyboardScrollView';
import TextInput from 'components/TextInput';
import fonts from 'styles/fonts';
import { withAccountStore } from 'utils/HOC';
import { navigateToLegacyAccountList } from 'utils/navigationHelpers';

interface State {
	confirmation: string;
	focusConfirmation: boolean;
	pin: string;
	pinMismatch: boolean;
	pinTooShort: boolean;
}
function AccountPin({
	accounts,
	navigation
}: NavigationAccountProps<{ isNew: boolean }>): React.ReactElement {
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
		const { pin, confirmation } = state;
		const accountCreation: boolean = navigation.getParam('isNew', false);
		const account = accountCreation
			? accounts.getNew()
			: accounts.getSelected()!;
		if (pin.length >= 6 && pin === confirmation) {
			if (accountCreation) {
				await accounts.submitNew(pin);
				return navigateToLegacyAccountList(navigation);
			} else {
				await accounts.save(accounts.getSelectedKey(), account, pin);
				const resetAction = StackActions.reset({
					actions: [
						NavigationActions.navigate({ routeName: 'LegacyAccountList' }),
						NavigationActions.navigate({ routeName: 'AccountDetails' })
					],
					index: 1, // FIXME workaround for now, use SwitchNavigator later: https://github.com/react-navigation/react-navigation/issues/1127#issuecomment-295841343
					key: undefined
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
			<Background />
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

export default withAccountStore(AccountPin);

const styles = StyleSheet.create({
	body: {
		backgroundColor: colors.bg,
		flex: 1,
		overflow: 'hidden',
		padding: 20
	},
	errorText: {
		color: colors.bg_alert,
		fontFamily: fonts.bold,
		fontSize: 12,
		paddingBottom: 20,
		textAlign: 'center'
	},
	hintText: {
		color: colors.bg_text_sec,
		fontFamily: fonts.bold,
		fontSize: 12,
		paddingBottom: 20,
		textAlign: 'center'
	},
	pinInput: {
		marginBottom: 20
	},
	title: {
		color: colors.bg_text_sec,
		fontFamily: fonts.bold,
		fontSize: 18,
		paddingBottom: 10
	},
	titleTop: {
		color: colors.bg_text_sec,
		fontFamily: fonts.bold,
		fontSize: 24,
		paddingBottom: 20,
		textAlign: 'center'
	}
});
