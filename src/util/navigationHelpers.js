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

import { NavigationActions, StackActions } from 'react-navigation';

export const setPin = async navigation =>
	new Promise(resolve => {
		navigation.navigate('IdentityPin', { isNew: true, resolve });
	});

export const unlockSeed = async (navigation, identity) =>
	new Promise(resolve => {
		navigation.navigate('IdentityPin', { identity, isUnlock: true, resolve });
	});

export const navigateToPathsList = (navigation, networkKey) => {
	const resetAction = StackActions.reset({
		actions: [
			NavigationActions.navigate({
				isNew: false,
				routeName: 'AccountNetworkChooser'
			}),
			NavigationActions.navigate({
				params: { networkKey },
				routeName: 'PathsList'
			})
		],
		index: 1,
		key: undefined
	});
	navigation.dispatch(resetAction);
};

export const navigateToLandingPage = (navigation, isSwitchOpen) => {
	const resetAction = StackActions.reset({
		actions: [
			NavigationActions.navigate({
				params: { isSwitchOpen },
				routeName: 'AccountNetworkChooser'
			})
		],
		index: 0,
		key: undefined
	});
	navigation.dispatch(resetAction);
};

export const navigateToNewIdentityNetwork = navigation => {
	const resetAction = StackActions.reset({
		actions: [
			NavigationActions.navigate({
				params: { isNew: true },
				routeName: 'AccountNetworkChooser'
			})
		],
		index: 0,
		key: undefined
	});
	navigation.dispatch(resetAction);
};

export const navigateToSignedMessage = navigation => {
	const resetAction = StackActions.reset({
		actions: [
			NavigationActions.navigate({
				isNew: false,
				routeName: 'AccountNetworkChooser'
			}),
			NavigationActions.navigate({
				params: { isNew: true },
				routeName: 'SignedMessage'
			})
		],
		index: 1,
		key: undefined
	});
	navigation.dispatch(resetAction);
};

export const navigateToSignedTx = navigation => {
	const resetAction = StackActions.reset({
		actions: [
			NavigationActions.navigate({
				isNew: false,
				routeName: 'AccountNetworkChooser'
			}),
			NavigationActions.navigate({
				params: { isNew: true },
				routeName: 'SignedTx'
			})
		],
		index: 1,
		key: undefined
	});
	navigation.dispatch(resetAction);
};

export const navigateToLegacyAccountList = navigation => {
	const resetAction = StackActions.reset({
		actions: [NavigationActions.navigate({ routeName: 'LegacyAccountList' })],
		index: 0,
		key: undefined
	});
	navigation.dispatch(resetAction);
};
