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

import React, { useRef, useState, useMemo, useContext } from 'react';

import PasswordInput from 'components/PasswordInput';
import testIDs from 'e2e/testIDs';
import { defaultNetworkKey, UnknownNetworkKeys } from 'constants/networkSpecs';
import { AlertStateContext } from 'stores/alertContext';
import { NetworksContext } from 'stores/NetworkContext';
import { NavigationAccountIdentityProps } from 'types/props';
import TextInput from 'components/TextInput';
import { withCurrentIdentity } from 'utils/HOC';
import {
	extractPathId,
	getNetworkKey,
	getSubstrateNetworkKeyByPathId,
	validateDerivedPath
} from 'utils/identitiesUtils';
import { unlockSeedPhrase } from 'utils/navigationHelpers';
import { alertPathDerivationError } from 'utils/alertUtils';
import Separator from 'components/Separator';
import ScreenHeading from 'components/ScreenHeading';
import PathCard from 'components/PathCard';
import {
	DerivationNetworkSelector,
	NetworkOptions
} from 'components/DerivationNetworkSelector';
import { useSeedRef } from 'utils/seedRefHooks';
import Button from 'components/Button';
import { KeyboardAwareContainer } from 'modules/unlock/components/Container';

function PathDerivation({
	accountsStore,
	navigation,
	route
}: NavigationAccountIdentityProps<'PathDerivation'>): React.ReactElement {
	const [derivationPath, setDerivationPath] = useState<string>('');
	const [keyPairsName, setKeyPairsName] = useState<string>('');
	const [modalVisible, setModalVisible] = useState<boolean>(false);
	const [password, setPassword] = useState<string>('');
	const networkContextState = useContext(NetworksContext);
	const pathNameInput = useRef<TextInput>(null);
	const { setAlert } = useContext(AlertStateContext);
	const currentIdentity = accountsStore.state.currentIdentity;
	const { isSeedRefValid, substrateAddress } = useSeedRef(
		currentIdentity.encryptedSeed
	);
	const parentPath = route.params.parentPath;

	const parentNetworkKey = useMemo((): string => {
		const { networks, pathIds } = networkContextState;
		function getParentNetworkKey(): string {
			if (currentIdentity.meta.has(parentPath)) {
				return getNetworkKey(parentPath, currentIdentity, networkContextState);
			}
			const pathId = extractPathId(parentPath, pathIds);
			return getSubstrateNetworkKeyByPathId(pathId, networks);
		}
		return getParentNetworkKey();
	}, [currentIdentity, networkContextState, parentPath]);

	const [customNetworkKey, setCustomNetworkKey] = useState(
		parentNetworkKey === UnknownNetworkKeys.UNKNOWN
			? defaultNetworkKey
			: parentNetworkKey
	);
	const completePath = `${parentPath}${derivationPath}`;
	const enableCustomNetwork = parentPath === '';
	const currentNetworkKey = enableCustomNetwork
		? customNetworkKey
		: parentNetworkKey;
	const isPathValid = validateDerivedPath(derivationPath);

	const onPathDerivation = async (): Promise<void> => {
		await unlockSeedPhrase(navigation, isSeedRefValid);
		try {
			await accountsStore.deriveNewPath(
				completePath,
				substrateAddress,
				networkContextState.getSubstrateNetwork(currentNetworkKey),
				keyPairsName,
				password
			);
			setAlert('Success', 'New Account Successfully derived');
			navigation.goBack();
		} catch (error) {
			alertPathDerivationError(setAlert, error.message);
		}
	};

	return (
		<KeyboardAwareContainer>
			<ScreenHeading
				title="Derive Account"
				subtitle={parentPath}
				hasSubtitleIcon={true}
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
				password={password}
				setPassword={setPassword}
				onSubmitEditing={onPathDerivation}
			/>
			<PathCard
				identity={accountsStore.state.currentIdentity!}
				isPathValid={isPathValid}
				name={keyPairsName}
				path={password === '' ? completePath : `${completePath}///${password}`}
				networkKey={currentNetworkKey}
			/>
			<Button
				disabled={!isPathValid}
				title="Next"
				testID={testIDs.PathDerivation.deriveButton}
				onPress={onPathDerivation}
			/>
			{enableCustomNetwork && (
				<NetworkOptions
					setNetworkKey={setCustomNetworkKey}
					visible={modalVisible}
					setVisible={setModalVisible}
				/>
			)}
		</KeyboardAwareContainer>
	);
}

export default withCurrentIdentity(PathDerivation);
