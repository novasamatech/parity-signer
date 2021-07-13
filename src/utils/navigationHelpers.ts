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

export const navigateToPathDetails = <
	RouteName extends keyof RootStackParamList
>(
	navigation: GenericNavigationProps<RouteName>,
	networkKey: string,
	path: string
): void => {
	const resetAction = CommonActions.reset({
		index: 2,
		routes: [
			{
				name: 'Main',
				params: { isNew: false }
			},
			{
				name: 'PathsList',
				params: { networkKey }
			},
			{
				name: 'PathDetails',
				params: { path }
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
	params: Record<string, unknown> = {},
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

export const navigateToNetworkSettings = <
	RouteName extends keyof RootStackParamList
>(
	navigation: GenericNavigationProps<RouteName>
): void => resetNavigationWithNetworkChooser(navigation, 'NetworkSettings');

export const navigateToPathsList = <RouteName extends keyof RootStackParamList>(
	navigation: GenericNavigationProps<RouteName>,
	networkKey: string
): void =>
	resetNavigationWithNetworkChooser(navigation, 'PathsList', { networkKey });

export const navigateToQrScanner = <RouteName extends keyof RootStackParamList>(
	navigation: GenericNavigationProps<RouteName>
): void => resetNavigationWithNetworkChooser(navigation, 'FastQrScanner');

export const navigateToFastQrScanner = <
	RouteName extends keyof RootStackParamList
>(
	navigation: GenericNavigationProps<RouteName>
): void => resetNavigationWithNetworkChooser(navigation, 'FastQrScanner');
