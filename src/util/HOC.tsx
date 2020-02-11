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
import AccountsStore from '../stores/AccountsStore';
import ScannerStore from '../stores/ScannerStore';

interface WithAccountProps {
	accounts: AccountsStore;
}

interface WithScannerProps {
	scanner: ScannerStore;
}

type WithAccountAndScannerProps = WithAccountProps & WithScannerProps;

export function withAccountStore<T extends WithAccountProps = WithAccountProps>(
	WrappedComponent: React.ComponentType<T>
) {
	return (props: T): React.ReactElement => (
		<Subscribe to={[AccountsStore]}>
			{(accounts: AccountsStore): React.ReactElement => (
				<WrappedComponent {...props} accounts={accounts} />
			)}
		</Subscribe>
	);
}

export function withScannerStore<T extends WithScannerProps = WithScannerProps>(
	WrappedComponent: React.ComponentType<T>
) {
	return (props: T): React.ReactElement => (
		<Subscribe to={[ScannerStore]}>
			{scanner => <WrappedComponent {...props} scanner={scanner} />}
		</Subscribe>
	);
}

export function withAccountAndScannerStore<
	T extends WithAccountAndScannerProps = WithAccountAndScannerProps
>(WrappedComponent: React.ComponentType<T>) {
	return (props: T): React.ReactElement => (
		<Subscribe to={[ScannerStore, AccountsStore]}>
			{(scanner, accounts) => (
				<WrappedComponent {...props} scanner={scanner} accounts={accounts} />
			)}
		</Subscribe>
	);
}
