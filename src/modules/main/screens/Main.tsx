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

import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import NoCurrentIdentity from 'modules/main/components/NoCurrentIdentity';
import OnBoardingView from 'modules/main/components/OnBoading';
import React, { useContext } from 'react';
import { NavigationAccountIdentityProps, NavigationProps } from 'types/props';

import { AccountsContext } from '../../../context';
import AccountSelector from '../components/AccountSelector';
import NetworkSelector from '../components/NetworkSelector';

export default function Main(props: NavigationProps<'Main'>): React.ReactElement {
	const isNew = props.route.params?.isNew ?? false;
	const accountsStore = useContext(AccountsContext);
	const { accounts, currentIdentity, identities, loaded } = accountsStore.state;
	const hasLegacyAccount = accounts.length !== 0;

	if (!loaded) return <SafeAreaViewContainer />;
	if (identities.length === 0)

		return <OnBoardingView hasLegacyAccount={hasLegacyAccount} />;
	if (currentIdentity === null) return <NoCurrentIdentity />;

	return (
		isNew
			? <NetworkSelector {...(props as NavigationAccountIdentityProps<'Main'>)} />
			: <AccountSelector {...(props as NavigationAccountIdentityProps<'Main'>)} />
	);
}
