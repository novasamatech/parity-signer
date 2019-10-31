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

import React, { Component, useState } from 'react';
import { StyleSheet, Text } from 'react-native';
import { Subscribe } from 'unstated';
import colors from '../colors';
import fonts from '../fonts';
import Background from '../components/Background';
import Button from '../components/Button';
import TextInput from '../components/TextInput';
import AccountsStore from '../stores/AccountsStore';
import KeyboardScrollView from '../components/KeyboardScrollView';

export default class IdentityPin extends React.PureComponent {
	render() {
		return (
			<Subscribe to={[AccountsStore]}>
				{accounts => <IdentityPinView {...this.props} accounts={accounts} />}
			</Subscribe>
		);
	}
}

function IdentityPinView({ navigation, accounts }) {
	const initialState = {
		confirmation: '',
		focusConfirmation: false,
		pin: '',
		pinMismatch: false,
		pinTooShort: false
	};
	const [state, setState] = useState(initialState);
	const updateState = delta => setState({ ...state, ...delta });

	const submit = async () => {
		const isIdentityCreation = navigation.getParam('isNew');
		const { pin, confirmation } = state;
		if (pin.length >= 6 && pin === confirmation) {
			if (isIdentityCreation) {
				const resolve = navigation.getParam('resolve');
				setState(initialState);
				resolve(pin);
			} else {
				// await accounts.save(accounts.getSelectedKey(), account, pin);
				// const resetAction = StackActions.reset({
				// 	actions: [
				// 		NavigationActions.navigate({ routeName: 'AccountList' }),
				// 		NavigationActions.navigate({ routeName: 'AccountDetails' })
				// 	],
				// 	index: 1, // FIXME workaround for now, use SwitchNavigator later: https://github.com/react-navigation/react-navigation/issues/1127#issuecomment-295841343
				// 	key: undefined
				// });
				// navigation.dispatch(resetAction);
			}
		} else {
			if (pin.length < 6) {
				updateState({ pinTooShort: true });
			} else if (pin !== confirmation) updateState({ pinMismatch: true });
		}
	};

	const testPin = async () => {
		const { pin } = state;
		if (pin.length >= 6) {
			try {
				const seed = await accounts.unlockIdentitySeed(pin);
				const resolve = navigation.getParam('resolve');
				setState(initialState);
				resolve(seed);
			} catch (e) {
				updateState({ pin: '', pinMismatch: true });
				//TODO record error times;
			}
		} else {
			updateState({ pinTooShort: true });
		}
	};

	const showHintOrError = () => {
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

	const onPinInputChange = (stateName, pinInput) => {
		if (/^\d+$|^$/.test(pinInput)) {
			updateState({
				pinMismatch: false,
				pinTooShort: false,
				[stateName]: pinInput
			});
		}
	};

	const title = 'ACCOUNT PIN';

	const renderPinInput = () =>
		navigation.getParam('isUnlock', false) ? (
			<>
				<Text style={styles.titleTop}>{title}</Text>
				{showHintOrError()}
				<Text style={styles.title}>PIN</Text>
				<PinInput
					autoFocus
					returnKeyType="done"
					onChangeText={pin => onPinInputChange('pin', pin)}
					value={state.pin}
				/>
				<Button
					onPress={testPin}
					color="green"
					title="Done"
					accessibilityLabel={'Done'}
				/>
			</>
		) : (
			<>
				<Text style={styles.titleTop}>{title}</Text>
				{showHintOrError()}
				<Text style={styles.title}>PIN</Text>
				<PinInput
					autoFocus
					returnKeyType="next"
					onFocus={() => updateState({ focusConfirmation: false })}
					onSubmitEditing={() => {
						updateState({ focusConfirmation: true });
					}}
					onChangeText={pin => onPinInputChange('pin', pin)}
					value={state.pin}
				/>
				<Text style={styles.title}>CONFIRM PIN</Text>
				<PinInput
					returnKeyType="done"
					focus={state.focusConfirmation}
					onChangeText={confirmation =>
						onPinInputChange('confirmation', confirmation)
					}
					value={state.confirmation}
				/>
				<Button
					onPress={submit}
					color="green"
					title="Done"
					accessibilityLabel={'Done'}
				/>
			</>
		);

	return (
		<KeyboardScrollView style={styles.body} extraHeight={120}>
			<Background />
			{renderPinInput()}
		</KeyboardScrollView>
	);
}

class PinInput extends Component {
	render() {
		return (
			<TextInput
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
				style={styles.pinInput}
				{...this.props}
			/>
		);
	}
}

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
