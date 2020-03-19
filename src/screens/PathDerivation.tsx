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

import React, { useRef, useState, useMemo } from 'react';
import { Platform, StyleSheet, Text, View } from 'react-native';

import testIDs from 'e2e/testIDs';
import { defaultNetworkKey, UnknownNetworkKeys } from 'constants/networkSpecs';
import { NavigationAccountProps } from 'types/props';
import { withAccountStore } from 'utils/HOC';
import TextInput from 'components/TextInput';
import ButtonMainAction from 'components/ButtonMainAction';
import {
	getNetworkKey,
	getNetworkKeyByPath,
	validateDerivedPath
} from 'utils/identitiesUtils';
import { navigateToPathsList, unlockSeedPhrase } from 'utils/navigationHelpers';
import { alertPathDerivationError } from 'utils/alertUtils';
import Separator from 'components/Separator';
import ScreenHeading from 'components/ScreenHeading';
import colors from 'styles/colors';
import PathCard from 'components/PathCard';
import KeyboardScrollView from 'components/KeyboardScrollView';
import { NetworkSelector, NetworkOptions } from 'components/NetworkSelector';

function PathDerivation({
	accounts,
	navigation,
	route
}: NavigationAccountProps<'PathDerivation'>): React.ReactElement {
	const [derivationPath, setDerivationPath] = useState('');
	const [keyPairsName, setKeyPairsName] = useState('');
	const [isPathValid, setIsPathValid] = useState(true);
	const [modalVisible, setModalVisible] = useState(false);
	const pathNameInput = useRef<TextInput>(null);
	const parentPath = route.params.parentPath;
	const [customNetworkKey, setCustomNetworkKey] = useState(() => {
		const parentNetworkKey = getNetworkKey(
			parentPath,
			accounts.state.currentIdentity!
		);
		return parentNetworkKey === UnknownNetworkKeys.UNKNOWN
			? defaultNetworkKey
			: parentNetworkKey;
	});
	const completePath = `${parentPath}${derivationPath}`;
	const pathIndicatedNetworkKey = useMemo(
		(): string => getNetworkKeyByPath(completePath),
		[completePath]
	);
	const isCustomNetwork =
		pathIndicatedNetworkKey === UnknownNetworkKeys.UNKNOWN;

	const onPathDerivation = async (): Promise<void> => {
		if (!validateDerivedPath(derivationPath)) {
			return setIsPathValid(false);
		}
		const seedPhrase = await unlockSeedPhrase(navigation);
		const derivationSucceed = await accounts.deriveNewPath(
			completePath,
			seedPhrase,
			isCustomNetwork ? customNetworkKey : pathIndicatedNetworkKey,
			keyPairsName
		);
		if (derivationSucceed) {
			navigateToPathsList(navigation, pathIndicatedNetworkKey);
		} else {
			setIsPathValid(false);
			alertPathDerivationError();
		}
	};

	return (
		<View style={styles.container}>
			<KeyboardScrollView extraHeight={Platform.OS === 'ios' ? 250 : 180}>
				<ScreenHeading
					title="Derive Account"
					subtitle={parentPath}
					hasSubtitleIcon={true}
				/>
				{!isPathValid && <Text>Invalid Path</Text>}
				<TextInput
					autoCompleteType="off"
					autoCorrect={false}
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
				{isCustomNetwork && (
					<NetworkSelector
						networkKey={customNetworkKey}
						setVisible={setModalVisible}
					/>
				)}
				<Separator style={{ height: 0 }} />
				<PathCard
					identity={accounts.state.currentIdentity!}
					name={keyPairsName}
					path={completePath}
				/>

				<ButtonMainAction
					disabled={!validateDerivedPath(derivationPath)}
					bottom={false}
					style={{ marginTop: 8 }}
					title="Next"
					testID={testIDs.PathDerivation.deriveButton}
					onPress={onPathDerivation}
				/>
				{isCustomNetwork && (
					<NetworkOptions
						setNetworkKey={setCustomNetworkKey}
						visible={modalVisible}
						setVisible={setModalVisible}
					/>
				)}
			</KeyboardScrollView>
		</View>
	);
}

const styles = StyleSheet.create({
	container: {
		backgroundColor: colors.bg,
		flex: 1
	}
});

export default withAccountStore(PathDerivation);
