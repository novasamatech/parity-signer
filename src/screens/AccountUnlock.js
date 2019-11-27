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

import PropTypes from 'prop-types';
import React from 'react';
import { Alert, StyleSheet, View } from 'react-native';
import { NavigationActions, StackActions } from 'react-navigation';
import { Subscribe } from 'unstated';
import colors from '../colors';
import fontStyles from '../fontStyles';
import Background from '../components/Background';
import ScreenHeading from '../components/ScreenHeading';
import TextInput from '../components/TextInput';
import AccountsStore from '../stores/AccountsStore';
import ScannerStore from '../stores/ScannerStore';

export class AccountUnlockAndSign extends React.PureComponent {
	render() {
		const { navigation } = this.props;
		const next = navigation.getParam('next', 'SignedTx');

		return (
			<Subscribe to={[AccountsStore, ScannerStore]}>
				{(accounts, scannerStore) => (
					<AccountUnlockView
						{...this.props}
						accounts={accounts}
						checkPin={async pin => {
							try {
								await scannerStore.signDataLegacy(pin);
								return true;
							} catch (e) {
								return false;
							}
						}}
						navigate={() => {
							const resetAction = StackActions.reset({
								actions: [
									NavigationActions.navigate({
										routeName: 'LegacyAccountList'
									}),
									NavigationActions.navigate({ routeName: next })
								],
								index: 1,
								key: undefined // FIXME workaround for now, use SwitchNavigator later: https://github.com/react-navigation/react-navigation/issues/1127#issuecomment-295841343
							});
							navigation.dispatch(resetAction);
						}}
					/>
				)}
			</Subscribe>
		);
	}
}

export class AccountUnlock extends React.PureComponent {
	render() {
		const { navigation } = this.props;
		const next = navigation.getParam('next', 'LegacyAccountList');

		return (
			<Subscribe to={[AccountsStore]}>
				{accounts => (
					<AccountUnlockView
						{...this.props}
						checkPin={async pin => {
							return await accounts.unlockAccount(
								accounts.getSelectedKey(),
								pin
							);
						}}
						navigate={async pin => {
							if (next === 'AccountBiometric') {
								navigation.goBack();
								if (accounts.getSelected().biometricEnabled) {
									await accounts.disableBiometric(accounts.getSelectedKey());
								} else {
									await accounts
										.enableBiometric(accounts.getSelectedKey(), pin)
										.catch(error => {
											// error here is likely no fingerprints/biometrics enrolled, so should be displayed to the user
											console.log(error);
											Alert.alert('Biometric Error', error.message, [
												{
													style: 'default',
													text: 'Ok'
												}
											]);
										});
								}
							} else if (next === 'AccountDelete') {
								navigation.goBack();
								navigation.state.params.onDelete();
							} else {
								const resetAction = StackActions.reset({
									actions: [
										NavigationActions.navigate({
											routeName: 'LegacyAccountList'
										}),
										NavigationActions.navigate({ routeName: 'AccountDetails' }),
										NavigationActions.navigate({
											params: next === 'AccountBiometric' ? { pin: pin } : {},
											routeName: next
										})
									],
									index: 2,
									key: undefined // FIXME workaround for now, use SwitchNavigator later: https://github.com/react-navigation/react-navigation/issues/1127#issuecomment-295841343
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

class AccountUnlockView extends React.PureComponent {
	static propTypes = {
		checkPin: PropTypes.func.isRequired,
		hasWrongPin: PropTypes.bool
	};

	state = {
		hasWrongPin: false,
		pin: ''
	};

	showErrorMessage = () => {
		return this.state.hasWrongPin ? 'Wrong pin, please try again' : '';
	};

	render() {
		const { checkPin, navigate } = this.props;

		return (
			<View style={styles.body}>
				<Background />
				<ScreenHeading
					title={'Unlock Account'}
					subtitle={this.showErrorMessage()}
					error={{ hasWrongPin: true }}
				/>
				<PinInput
					label="PIN"
					onChangeText={async pin => {
						this.setState({ pin: pin });
						if (pin.length < 4) {
							return;
						}
						if (await checkPin(pin)) {
							navigate(pin);
						} else if (pin.length > 5) {
							this.setState({ hasWrongPin: true });
						}
					}}
					value={this.state.pin}
				/>
			</View>
		);
	}
}

function PinInput(props) {
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
