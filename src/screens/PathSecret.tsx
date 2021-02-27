// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// Modifications Copyright (c) 2021 Thibaut Sardan

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

import PathCard from 'components/PathCard';
import QrView from 'components/QrView';
import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import ScreenHeading from 'components/ScreenHeading';
import { PasswordedAccountExportWarning } from 'components/Warnings';
import testIDs from 'e2e/testIDs';
import React, { useContext, useEffect, useState } from 'react';
import { ScrollView } from 'react-native';
import { NavigationAccountIdentityProps } from 'types/props';
import { withCurrentIdentity } from 'utils/HOC';
import { getNetworkKey, getPathName } from 'utils/identitiesUtils';
import { useSeedRef } from 'utils/seedRefHooks';

import { NetworksContext } from '../context';

function PathSecret({ accountsStore, navigation, route }: NavigationAccountIdentityProps<'PathSecret'>): React.ReactElement {
	const networksContextState = useContext(NetworksContext);
	const { currentIdentity } = accountsStore.state;
	const [secret, setSecret] = useState<string>('');
	const { isSeedRefValid, substrateSecret } = useSeedRef(currentIdentity.encryptedSeed);
	const path = route.params.path;
	const pathMeta = currentIdentity.meta.get(path)!;

	useEffect(() => {
		const getAndSetSecret = async (): Promise<void> => {
			const networkKey = getNetworkKey(path,
				currentIdentity,
				networksContextState);
			const password = route.params.password ?? '';
			const accountName = getPathName(path, currentIdentity);
			const generatedSecret = await substrateSecret(`${path}///${password}`);

			setSecret(`secret:0x${generatedSecret}:${networkKey}:${accountName}`);
		};

		getAndSetSecret();
	}, [
		path,
		pathMeta,
		route.params.password,
		navigation,
		currentIdentity,
		isSeedRefValid,
		substrateSecret,
		networksContextState
	]);

	return (
		<SafeAreaViewContainer>
			<ScreenHeading
				subtitle={
					'Export this account to an hot machine, keep this QR safe, the QR allows any one to recover the account and access its fund'
				}
				title={'Export Account'}
			/>
			<ScrollView bounces={false}
				testID={testIDs.PathSecret.screen}>
				<PathCard identity={currentIdentity}
					path={path} />
				<QrView data={secret}
					testID={secret} />
				{pathMeta.hasPassword && <PasswordedAccountExportWarning />}
			</ScrollView>
		</SafeAreaViewContainer>
	);
}

export default withCurrentIdentity(PathSecret);
