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

import React, { useContext } from 'react';

import NoCurrentIdentity from 'modules/main/components/NoCurrentIdentity';
import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import OnBoardingView from 'modules/main/components/OnBoading';
import NetworkSelector from 'modules/main/components/NetworkSelector';
import { AccountsContext } from 'stores/AccountsContext';
import { NavigationAccountIdentityProps, NavigationProps } from 'types/props';

export default function Main(
	props: NavigationProps<'Main'>
): React.ReactElement {
	const accountsStore = useContext(AccountsContext);
	const { identities, currentIdentity, loaded, accounts } = accountsStore.state;
	const hasLegacyAccount = accounts.size !== 0;

	if (!loaded) return <SafeAreaViewContainer />;
	if (identities.length === 0)
		return <OnBoardingView hasLegacyAccount={hasLegacyAccount} />;
	if (currentIdentity === null) return <NoCurrentIdentity />;
	return (
		<NetworkSelector {...(props as NavigationAccountIdentityProps<'Main'>)} />
	);
}
