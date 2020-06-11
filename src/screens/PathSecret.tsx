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

import React, { useEffect, useState } from 'react';
import { ScrollView } from 'react-native';

import PathCard from 'components/PathCard';
import QrView from 'components/QrView';
import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import { PasswordedAccountExportWarning } from 'components/Warnings';
import testIDs from 'e2e/testIDs';
import { NavigationAccountIdentityProps } from 'types/props';
import { withAccountStore, withCurrentIdentity } from 'utils/HOC';
import { getPathName } from 'utils/identitiesUtils';
import {
	unlockSeedPhrase,
	unlockSeedPhraseWithPassword
} from 'utils/navigationHelpers';
import { useSeedRef } from 'utils/seedRefHooks';

function PathSecret({
	accounts,
	route,
	navigation
}: NavigationAccountIdentityProps<'PathSecret'>): React.ReactElement {
	const { currentIdentity } = accounts.state;
	const [secret, setSecret] = useState<string>('');
	const { substrateSecret, isSeedRefValid } = useSeedRef(
		currentIdentity.encryptedSeed
	);
	const path = route.params.path;
	const pathMeta = currentIdentity.meta.get(path)!;
	const accountName = getPathName(path, currentIdentity);

	const getAndSetSecret = async (): Promise<void> => {
		let generatedSecret;
		let password = route.params.password ?? '';
		if (pathMeta.hasPassword) {
			password = await unlockSeedPhraseWithPassword(navigation, isSeedRefValid);
		} else {
			if (!isSeedRefValid) {
				await unlockSeedPhrase(navigation, isSeedRefValid);
			}
		}
		generatedSecret = await substrateSecret(`${path}///${password}`);
		setSecret(`secret:${generatedSecret}:${accountName}`);
	};

	useEffect(() => {
		getAndSetSecret();
	}, [route, pathMeta, navigation, isSeedRefValid, substrateSecret]);

	return (
		<SafeAreaViewContainer>
			<ScrollView testID={testIDs.PathSecret.screen} bounces={false}>
				<PathCard identity={currentIdentity} path={path} />
				<QrView data={secret} />
				{pathMeta.hasPassword && <PasswordedAccountExportWarning />}
			</ScrollView>
		</SafeAreaViewContainer>
	);
}

export default withAccountStore(withCurrentIdentity(PathSecret));
