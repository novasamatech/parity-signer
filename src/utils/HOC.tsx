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
import { Subscribe } from 'unstated';

import AccountsStore from 'stores/AccountsStore';
import RegistriesStore from 'stores/RegistriesStore';
import ScannerStore from 'stores/ScannerStore';

interface AccountInjectedProps {
	accounts: AccountsStore;
}

interface ScannerInjectedProps {
	scanner: ScannerStore;
}

interface RegistriesInjectedProps {
	registriesStore: RegistriesStore;
}

type AccountAndScannerInjectedProps = AccountInjectedProps &
	ScannerInjectedProps;

export function withAccountStore<T extends AccountInjectedProps>(
	WrappedComponent: React.ComponentType<any>
): React.ComponentType<Omit<T, keyof AccountInjectedProps>> {
	return (props): React.ReactElement => (
		<Subscribe to={[AccountsStore]}>
			{(accounts: AccountsStore): React.ReactElement => (
				<WrappedComponent accounts={accounts} {...props} />
			)}
		</Subscribe>
	);
}

export function withScannerStore<T extends ScannerInjectedProps>(
	WrappedComponent: React.ComponentType<any>
): React.ComponentType<Omit<T, keyof AccountInjectedProps>> {
	return (props): React.ReactElement => (
		<Subscribe to={[ScannerStore]}>
			{(scannerStore): React.ReactElement => (
				<WrappedComponent {...props} scannerStore={scannerStore} />
			)}
		</Subscribe>
	);
}

export function withAccountAndScannerStore<
	T extends AccountAndScannerInjectedProps
>(
	WrappedComponent: React.ComponentType<any>
): React.ComponentType<Omit<T, keyof AccountAndScannerInjectedProps>> {
	return (props): React.ReactElement => (
		<Subscribe to={[ScannerStore, AccountsStore]}>
			{(scannerStore, accounts: AccountsStore): React.ReactElement => (
				<WrappedComponent
					{...props}
					scannerStore={scannerStore}
					accounts={accounts}
				/>
			)}
		</Subscribe>
	);
}

export function withRegistriesStore<T extends RegistriesInjectedProps>(
	WrappedComponent: React.ComponentType<any>
): React.ComponentType<Omit<T, keyof RegistriesInjectedProps>> {
	return (props): React.ReactElement => (
		<Subscribe to={[RegistriesStore]}>
			{(registriesStore): React.ReactElement => (
				<WrappedComponent {...props} registriesStore={registriesStore} />
			)}
		</Subscribe>
	);
}
