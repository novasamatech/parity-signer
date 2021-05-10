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

import { RouteProp } from '@react-navigation/native';
import React, { useContext } from 'react';
import { View } from 'react-native';

import { AccountsContext } from 'stores/AccountsContext';
import { AccountsStoreStateWithIdentity, Identity } from 'types/identityTypes';
import { RootStackParamList } from 'types/routes';
import { NetworksContext, NetworksContextState } from 'stores/NetworkContext';

interface RegistriesInjectedProps {
	registriesStore: NetworksContextState;
}

export function withRegistriesStore<T extends RegistriesInjectedProps>(
	WrappedComponent: React.ComponentType<any>
): React.ComponentType<Omit<T, keyof RegistriesInjectedProps>> {
	return (props): React.ReactElement => {
		const registriesStore = useContext(NetworksContext);
		return <WrappedComponent {...props} registriesStore={registriesStore} />;
	};
}

export function withCurrentIdentity<
	T extends { accountsStore: AccountsStoreStateWithIdentity }
>(WrappedComponent: React.ComponentType<T>): React.ComponentType<T> {
	return (props): React.ReactElement => {
		const accountsStore = useContext(AccountsContext);
		const { currentIdentity } = accountsStore.state;
		if (currentIdentity === null) return <View />;
		return <WrappedComponent {...props} accountsStore={accountsStore} />;
	};
}

interface UnlockScreenProps {
	route:
		| RouteProp<RootStackParamList, 'PinUnlock'>
		| RouteProp<RootStackParamList, 'PinUnlockWithPassword'>;
	targetIdentity: Identity;
}

export function withTargetIdentity<T extends UnlockScreenProps>(
	WrappedComponent: React.ComponentType<T>
): React.ComponentType<T> {
	return (props): React.ReactElement => {
		const accountsStore = useContext(AccountsContext);
		const targetIdentity =
			props.route.params.identity ?? accountsStore.state.currentIdentity;
		if (!targetIdentity) return <View />;
		return <WrappedComponent {...props} targetIdentity={targetIdentity} />;
	};
}
