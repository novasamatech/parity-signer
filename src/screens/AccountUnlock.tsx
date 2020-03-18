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

import React from 'react';
import { StyleSheet } from 'react-native';
import { CommonActions } from '@react-navigation/native';
import { Subscribe } from 'unstated';

import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import { NavigationProps } from 'types/props';
import colors from 'styles/colors';
import fontStyles from 'styles/fontStyles';
import ScreenHeading from 'components/ScreenHeading';
import TextInput from 'components/TextInput';
import AccountsStore from 'stores/AccountsStore';
import ScannerStore from 'stores/ScannerStore';

/* Used for unlock and sign tx and messages for legacy accounts */
export class AccountUnlockAndSign extends React.PureComponent<
	NavigationProps<'AccountUnlockAndSign'>
> {
	render(): React.ReactElement {
		const { navigation, route } = this.props;
		const next = route.params.next ?? 'SignedTx';

		return (
			<Subscribe to={[AccountsStore, ScannerStore]}>
				{(
					accounts: AccountsStore,
					scannerStore: ScannerStore
				): React.ReactElement => (
					<AccountUnlockView
						{...this.props}
						checkPin={async (pin: string): Promise<boolean> => {
							try {
								await scannerStore.signDataLegacy(pin);
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
				)}
			</Subscribe>
		);
	}
}

export class AccountUnlock extends React.PureComponent<
	NavigationProps<'AccountUnlock'>
> {
	render(): React.ReactElement {
		const { navigation, route } = this.props;
		const next = route.params.next ?? 'LegacyAccountList';
		const onDelete = route.params.onDelete ?? ((): any => null);

		return (
			<Subscribe to={[AccountsStore]}>
				{(accounts: AccountsStore): React.ReactElement => (
					<AccountUnlockView
						{...this.props}
						checkPin={async (pin: string): Promise<boolean> => {
							return await accounts.unlockAccount(
								accounts.getSelectedKey(),
								pin
							);
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
								this.props.navigation.dispatch(resetAction);
							}
						}}
					/>
				)}
			</Subscribe>
		);
	}
}

interface AccountUnlockViewProps {
	checkPin: (pin: string) => Promise<boolean>;
	hasWrongPin?: boolean;
	navigate: () => void;
}

interface AccountUnlockViewState {
	hasWrongPin: boolean;
	pin: string;
}

class AccountUnlockView extends React.PureComponent<
	AccountUnlockViewProps,
	AccountUnlockViewState
> {
	state = {
		hasWrongPin: false,
		pin: ''
	};

	showErrorMessage = (): string => {
		return this.state.hasWrongPin ? 'Wrong pin, please try again' : '';
	};

	render(): React.ReactElement {
		const { checkPin, navigate } = this.props;
		const { hasWrongPin, pin } = this.state;

		return (
			<SafeAreaViewContainer style={styles.body}>
				<ScreenHeading
					title={'Unlock Account'}
					subtitle={this.showErrorMessage()}
					error={hasWrongPin}
				/>
				<PinInput
					label="PIN"
					onChangeText={async (inputPin: string): Promise<void> => {
						this.setState({ pin: inputPin });
						if (inputPin.length < 4) {
							return;
						}
						if (await checkPin(inputPin)) {
							navigate();
						} else if (inputPin.length > 5) {
							this.setState({ hasWrongPin: true });
						}
					}}
					value={pin}
				/>
			</SafeAreaViewContainer>
		);
	}
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
