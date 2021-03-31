// Copyright 2015-2020 Parity Technologies (UK) Ltd.
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

import {
	CommonActions,
	useNavigation,
	useNavigationState
} from '@react-navigation/native';
import { StackNavigationProp } from '@react-navigation/stack';

import { Identity } from 'types/identityTypes';
import { RootStackParamList } from 'types/routes';

type Route = {
	name: keyof RootStackParamList;
	params?: RootStackParamList[keyof RootStackParamList];
};

export type GenericNavigationProps<
	RouteName extends keyof RootStackParamList
> = StackNavigationProp<RootStackParamList, RouteName>;

export const setPin = async <RouteName extends keyof RootStackParamList>(
	navigation: GenericNavigationProps<RouteName>
): Promise<string> =>
	new Promise(resolve => {
		navigation.navigate('PinNew', { resolve });
	});

export const unlockAndReturnSeed = async <
	RouteName extends keyof RootStackParamList
>(
	navigation: GenericNavigationProps<RouteName>
): Promise<string> =>
	new Promise(resolve => {
		navigation.navigate('PinUnlock', {
			resolve,
			shouldReturnSeed: true
		});
	});

type Unlock = (nextRoute: Route, identity?: Identity) => Promise<void>;
export const useUnlockSeed = (isSeedRefValid: boolean): Unlock => {
	const currentRoutes = useNavigationState(state => state.routes) as Route[];
	const navigation = useNavigation<StackNavigationProp<RootStackParamList>>();
	const resetRoutes = (routes: Route[]): void => {
		const resetAction = CommonActions.reset({
			index: routes.length,
			routes: routes
		});
		navigation.dispatch(resetAction);
	};

	return async (nextRoute, identity): Promise<void> => {
		await unlockSeedPhrase(navigation, isSeedRefValid, identity);
		const newRoutes = currentRoutes.concat(nextRoute);
		resetRoutes(newRoutes);
	};
};

export const unlockSeedPhrase = async <
	RouteName extends keyof RootStackParamList
>(
	navigation: GenericNavigationProps<RouteName>,
	isSeedRefValid: boolean,
	identity?: Identity
): Promise<void> =>
	new Promise(resolve => {
		if (isSeedRefValid) {
			resolve();
		} else {
			navigation.navigate('PinUnlock', {
				identity,
				resolve,
				shouldReturnSeed: false
			});
		}
	});

export const unlockSeedPhraseWithPassword = async <
	RouteName extends keyof RootStackParamList
>(
	navigation: GenericNavigationProps<RouteName>,
	isSeedRefValid: boolean,
	identity?: Identity
): Promise<string> =>
	new Promise(resolve => {
		navigation.navigate('PinUnlockWithPassword', {
			identity,
			isSeedRefValid,
			resolve
		});
	});

export const navigateToPathDetails = <
	RouteName extends keyof RootStackParamList
>(
	navigation: GenericNavigationProps<RouteName>,
	networkKey: string,
	path: string
): void => {
	const resetAction = CommonActions.reset({
		index: 1,
		routes: [
			{
				name: 'Main',
				params: { isNew: false }
			},
			{
				name: 'PathDetails',
				params: { networkKey, path }
			}
		]
	});
	navigation.dispatch(resetAction);
};

export const navigateToLandingPage = <
	RouteName extends keyof RootStackParamList
>(
	navigation: GenericNavigationProps<RouteName>
): void => {
	const resetAction = CommonActions.reset({
		index: 0,
		routes: [{ name: 'Main' }]
	});
	navigation.dispatch(resetAction);
};

export const navigateToNewIdentityNetwork = <
	RouteName extends keyof RootStackParamList
>(
	navigation: GenericNavigationProps<RouteName>
): void => {
	const resetAction = CommonActions.reset({
		index: 0,
		routes: [
			{
				name: 'Main',
				params: { isNew: true }
			}
		]
	});
	navigation.dispatch(resetAction);
};

export const resetNavigationTo = <RouteName extends keyof RootStackParamList>(
	navigation: GenericNavigationProps<RouteName>,
	screenName: string,
	params?: any
): void => {
	const resetAction = CommonActions.reset({
		index: 0,
		routes: [{ name: screenName, params }]
	});
	navigation.dispatch(resetAction);
};

export const resetNavigationWithNetworkChooser = <
	RouteName extends keyof RootStackParamList
>(
	navigation: GenericNavigationProps<RouteName>,
	screenName: string,
	params: object = {},
	isNew = false
): void => {
	const resetAction = CommonActions.reset({
		index: 1,
		routes: [
			{
				name: 'Main',
				params: { isNew }
			},
			{
				name: screenName,
				params: params
			}
		]
	});
	navigation.dispatch(resetAction);
};

export const resetNavigationWithScanner = <
	RouteName extends keyof RootStackParamList
>(
	navigation: GenericNavigationProps<RouteName>,
	screenName: string
): void => {
	const resetAction = CommonActions.reset({
		index: 1,
		routes: [
			{
				name: 'Main',
				params: { isNew: false }
			},
			{
				name: 'QrScanner'
			},
			{
				name: screenName
			}
		]
	});
	navigation.dispatch(resetAction);
};

export const navigateToQrScanner = <RouteName extends keyof RootStackParamList>(
	navigation: GenericNavigationProps<RouteName>
): void => resetNavigationWithNetworkChooser(navigation, 'QrScanner');

export const navigateToMain = <RouteName extends keyof RootStackParamList>(
	navigation: GenericNavigationProps<RouteName>
): void => resetNavigationTo(navigation, 'Main');

export const navigateToIdentitySwitch = <
	RouteName extends keyof RootStackParamList
>(
	navigation: GenericNavigationProps<RouteName>
): void => resetNavigationTo(navigation, 'IdentitySwitch');
