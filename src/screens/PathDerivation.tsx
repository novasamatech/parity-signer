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

import Button from 'components/Button';
import { DerivationNetworkSelector, NetworkOptions } from 'components/DerivationNetworkSelector';
import PasswordInput from 'components/PasswordInput';
import PathCard from 'components/PathCard';
import ScreenHeading from 'components/ScreenHeading';
import Separator from 'components/Separator';
import TextInput from 'components/TextInput';
import { defaultNetworkKey, UnknownNetworkKeys } from 'constants/networkSpecs';
import testIDs from 'e2e/testIDs';
import { KeyboardAwareContainer } from 'modules/unlock/components/Container';
import React, { useContext,useMemo, useRef, useState } from 'react';
import { NavigationAccountIdentityProps } from 'types/props';
import { alertPathDerivationError } from 'utils/alertUtils';
import { withCurrentIdentity } from 'utils/HOC';
import { extractPathId, getNetworkKey, getSubstrateNetworkKeyByPathId, validateDerivedPath } from 'utils/identitiesUtils';
import { navigateToPathDetails, unlockSeedPhrase } from 'utils/navigationHelpers';
import { useSeedRef } from 'utils/seedRefHooks';

import { AlertContext, NetworksContext } from '../context';

function PathDerivation({ accountsStore, navigation, route }: NavigationAccountIdentityProps<'PathDerivation'>): React.ReactElement {
	const [derivationPath, setDerivationPath] = useState<string>('');
	const [keyPairsName, setKeyPairsName] = useState<string>('');
	const [modalVisible, setModalVisible] = useState<boolean>(false);
	const [password, setPassword] = useState<string>('');
	const networkContextState = useContext(NetworksContext);
	const pathNameInput = useRef<TextInput>(null);
	const { setAlert } = useContext(AlertContext);
	const currentIdentity = accountsStore.state.currentIdentity;
	const { isSeedRefValid, substrateAddress } = useSeedRef(currentIdentity.encryptedSeed);
	const parentPath = route.params.parentPath;

	const parentNetworkKey = useMemo((): string => {
		const { networks, pathIds } = networkContextState;

		if (currentIdentity.meta.has(parentPath)) {
			return getNetworkKey(parentPath, currentIdentity, networkContextState);
		}

		const pathId = extractPathId(parentPath, pathIds);

		return getSubstrateNetworkKeyByPathId(pathId, networks);
	}, [currentIdentity, networkContextState, parentPath]);

	const [customNetworkKey, setCustomNetworkKey] = useState(parentNetworkKey === UnknownNetworkKeys.UNKNOWN
		? defaultNetworkKey
		: parentNetworkKey);
	const completePath = `${parentPath}${derivationPath}`;
	const enableCustomNetwork = parentPath === '';
	const currentNetworkKey = enableCustomNetwork
		? customNetworkKey
		: parentNetworkKey;
	const isPathValid = validateDerivedPath(derivationPath);

	const onPathDerivation = async (): Promise<void> => {
		await unlockSeedPhrase(navigation, isSeedRefValid);

		try {
			await accountsStore.deriveNewPath(completePath,
				substrateAddress,
				networkContextState.getSubstrateNetwork(currentNetworkKey),
				keyPairsName,
				password);
			setAlert('Success', 'New Account Successfully derived');
			navigateToPathDetails(navigation, currentNetworkKey, derivationPath);
		} catch (error) {
			alertPathDerivationError(setAlert, error.message);
		}
	};

	return (
		<KeyboardAwareContainer>
			<ScreenHeading
				hasSubtitleIcon={true}
				subtitle={parentPath}
				title="Derive Account"
			/>
			<TextInput
				autoCompleteType="off"
				autoCorrect={false}
				autoFocus
				error={!isPathValid}
				label="Path"
				onChangeText={setDerivationPath}
				onSubmitEditing={(): void => pathNameInput.current?.input?.focus()}
				placeholder="//hard/soft"
				returnKeyType="next"
				testID={testIDs.PathDerivation.pathInput}
				value={derivationPath}
			/>
			<TextInput
				autoCompleteType="off"
				autoCorrect={false}
				label="Display Name"
				onChangeText={(keyParisName: string): void =>
					setKeyPairsName(keyParisName)
				}
				onSubmitEditing={onPathDerivation}
				ref={pathNameInput}
				returnKeyType="done"
				testID={testIDs.PathDerivation.nameInput}
				value={keyPairsName}
			/>
			{enableCustomNetwork && (
				<DerivationNetworkSelector
					networkKey={customNetworkKey}
					setVisible={setModalVisible}
				/>
			)}
			<Separator style={{ height: 0 }} />
			<PasswordInput
				onSubmitEditing={onPathDerivation}
				password={password}
				setPassword={setPassword}
			/>
			<PathCard
				identity={accountsStore.state.currentIdentity!}
				isPathValid={isPathValid}
				name={keyPairsName}
				networkKey={currentNetworkKey}
				path={password === '' ? completePath : `${completePath}///${password}`}
			/>
			<Button
				disabled={!isPathValid}
				onPress={onPathDerivation}
				testID={testIDs.PathDerivation.deriveButton}
				title="Next"
			/>
			{enableCustomNetwork && (
				<NetworkOptions
					setNetworkKey={setCustomNetworkKey}
					setVisible={setModalVisible}
					visible={modalVisible}
				/>
			)}
		</KeyboardAwareContainer>
	);
}

export default withCurrentIdentity(PathDerivation);
