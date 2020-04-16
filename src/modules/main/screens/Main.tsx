import React from 'react';

import NoCurrentIdentity from 'modules/main/components/NoCurrentIdentity';
import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import OnBoardingView from 'modules/main/components/OnBoading';
import NetworkSelector from 'modules/main/components/NetworkSelector';
import { NavigationAccountProps } from 'types/props';
import { withAccountStore } from 'utils/HOC';

function Main(props: NavigationAccountProps<'Main'>): React.ReactElement {
	const { identities, currentIdentity, loaded } = props.accounts.state;
	const hasLegacyAccount = props.accounts.getAccounts().size !== 0;

	if (!loaded) return <SafeAreaViewContainer />;
	if (identities.length === 0)
		return <OnBoardingView hasLegacyAccount={hasLegacyAccount} />;
	if (!currentIdentity) return <NoCurrentIdentity />;
	return <NetworkSelector {...props} />;
}

export default withAccountStore(Main);
