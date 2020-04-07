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

import React, { useState } from 'react';
import { StyleSheet, TextInputProps } from 'react-native';

import testIDs from 'e2e/testIDs';
import colors from 'styles/colors';
import ButtonMainAction from 'components/ButtonMainAction';
import TextInput from 'components/TextInput';
import KeyboardScrollView from 'components/KeyboardScrollView';
import { withAccountStore } from 'utils/HOC';
import ScreenHeading from 'components/ScreenHeading';
import fontStyles from 'styles/fontStyles';
import { onlyNumberRegex } from 'utils/regex';
import { unlockIdentitySeed } from 'utils/identitiesUtils';
import { NavigationAccountProps } from 'types/props';

interface Props extends NavigationAccountProps<'IdentityPin'>, TextInputProps {
	label: string;
}

type State = {
	confirmation: string;
	focusConfirmation: boolean;
	pin: string;
	pinMismatch: boolean;
	pinTooShort: boolean;
};
function IdentityPin({ accounts, route }: Props): React.ReactElement {
	const initialState: State = {
		confirmation: '',
		focusConfirmation: false,
		pin: '',
		pinMismatch: false,
		pinTooShort: false
	};
	const [state, setState] = useState(initialState);
	const updateState = (delta: Partial<State>): void =>
		setState({ ...state, ...delta });
	const isUnlock = route.params.isUnlock ?? false;

	const submit = async (): Promise<void> => {
		const isIdentityCreation = route.params.isNew ?? false;
		const { pin, confirmation } = state;
		if (pin.length >= 6 && pin === confirmation) {
			if (isIdentityCreation) {
				const resolve = route.params.resolve;
				setState(initialState);
				resolve(pin);
			}
		} else {
			if (pin.length < 6) {
				updateState({ pinTooShort: true });
			} else if (pin !== confirmation) updateState({ pinMismatch: true });
		}
	};

	const testPin = async (): Promise<void> => {
		const { pin } = state;
		const { currentIdentity } = accounts.state;
		if (pin.length >= 6 && currentIdentity) {
			try {
				const identity = route.params.identity ?? currentIdentity;
				const resolve = route.params.resolve;
				const seed = await unlockIdentitySeed(pin, identity);
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

	const showHintOrError = (): string => {
		if (state.pinTooShort) {
			return t.pinTooShortHint;
		} else if (state.pinMismatch) {
			return isUnlock
				? t.pinMisMatchHint.pinUnlock
				: t.pinMisMatchHint.pinCreation;
		}
		return isUnlock ? t.subtitle.pinUnlock : t.subtitle.pinCreation;
	};

	const onPinInputChange = (stateName: string, pinInput: string): void => {
		if (onlyNumberRegex.test(pinInput)) {
			updateState({
				pinMismatch: false,
				pinTooShort: false,
				[stateName]: pinInput
			});
		}
	};

	const renderPinInput = (): React.ReactElement =>
		isUnlock ? (
			<>
				<ScreenHeading
					title={t.title.pinUnlock}
					error={state.pinMismatch || state.pinTooShort}
					subtitle={showHintOrError()}
				/>
				<PinInput
					label={t.pinLabel}
					autoFocus
					testID={testIDs.IdentityPin.unlockPinInput}
					returnKeyType="done"
					onChangeText={(pin: string): void => onPinInputChange('pin', pin)}
					onSubmitEditing={testPin}
					value={state.pin}
				/>
				<ButtonMainAction
					title={t.doneButton.pinUnlock}
					bottom={false}
					onPress={testPin}
					testID={testIDs.IdentityPin.unlockPinButton}
				/>
			</>
		) : (
			<>
				<ScreenHeading
					title={t.title.pinCreation}
					subtitle={showHintOrError()}
					error={state.pinMismatch || state.pinTooShort}
				/>

				<PinInput
					label={t.pinLabel}
					autoFocus
					testID={testIDs.IdentityPin.setPin}
					returnKeyType="next"
					onFocus={(): void => updateState({ focusConfirmation: false })}
					onSubmitEditing={(): void => {
						updateState({ focusConfirmation: true });
					}}
					onChangeText={(pin: string): void => onPinInputChange('pin', pin)}
					value={state.pin}
				/>
				<PinInput
					label={t.pinConfirmLabel}
					returnKeyType="done"
					testID={testIDs.IdentityPin.confirmPin}
					focus={state.focusConfirmation}
					onChangeText={(confirmation: string): void =>
						onPinInputChange('confirmation', confirmation)
					}
					value={state.confirmation}
					onSubmitEditing={submit}
				/>
				<ButtonMainAction
					title={t.doneButton.pinCreation}
					bottom={false}
					onPress={submit}
					testID={testIDs.IdentityPin.submitButton}
				/>
			</>
		);

	return (
		<KeyboardScrollView
			style={styles.body}
			extraHeight={200}
			testID={testIDs.IdentityPin.scrollScreen}
		>
			{renderPinInput()}
		</KeyboardScrollView>
	);
}

interface PinInputProps extends TextInputProps {
	label: string;
	focus?: boolean;
}

function PinInput(props: PinInputProps): React.ReactElement {
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
			{...props}
			style={StyleSheet.flatten([
				fontStyles.t_seed,
				styles.pinInput,
				{ fontSize: 22 },
				props.style
			])}
		/>
	);
}

const t = {
	doneButton: {
		pinCreation: 'DONE',
		pinUnlock: 'UNLOCK'
	},
	pinConfirmLabel: 'Confirm PIN',
	pinLabel: 'PIN',
	pinMisMatchHint: {
		pinCreation: "Pin codes don't match!",
		pinUnlock: 'Pin code is wrong!'
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

export default withAccountStore(IdentityPin);

const styles = StyleSheet.create({
	body: {
		backgroundColor: colors.bg,
		flex: 1,
		overflow: 'hidden'
	},
	pinInput: {
		borderBottomColor: colors.bg_button,
		borderColor: colors.bg_button,
		minHeight: 48,
		paddingLeft: 10,
		paddingRight: 10
	}
});
