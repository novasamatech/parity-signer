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

'use strict';

import React, { useRef, useState, useMemo } from 'react';
import { withNavigation } from 'react-navigation';
import { withAccountStore } from '../util/HOC';
import { Platform, StyleSheet, Text, View } from 'react-native';
import TextInput from '../components/TextInput';
import ButtonMainAction from '../components/ButtonMainAction';
import {
	getNetworkKeyByPath,
	validateDerivedPath
} from '../util/identitiesUtils';
import {
	navigateToPathsList,
	unlockSeedPhrase
} from '../util/navigationHelpers';
import { alertPathDerivationError } from '../util/alertUtils';
import testIDs from '../../e2e/testIDs';
import Separator from '../components/Separator';
import ScreenHeading from '../components/ScreenHeading';
import colors from '../colors';
import PathCard from '../components/PathCard';
import KeyboardScrollView from '../components/KeyboardScrollView';
import { defaultNetworkKey, UnknownNetworkKeys } from '../constants';
import { NetworkSelector, NetworkOptions } from '../components/NetworkSelector';

function PathDerivation({ accounts, navigation }) {
	const [derivationPath, setDerivationPath] = useState('');
	const [keyPairsName, setKeyPairsName] = useState('');
	const [isPathValid, setIsPathValid] = useState(true);
	const [modalVisible, setModalVisible] = useState(false);
	const [customNetworkKey, setCustomNetworkKey] = useState(defaultNetworkKey);
	const pathNameInput = useRef(null);
	const parentPath = navigation.getParam('parentPath');
	const completePath = `${parentPath}${derivationPath}`;
	const pathIndicatedNetworkKey = useMemo(
		() => getNetworkKeyByPath(completePath),
		[completePath]
	);
	const isCustomNetwork =
		pathIndicatedNetworkKey === UnknownNetworkKeys.UNKNOWN;

	const onPathDerivation = async () => {
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
			<ScreenHeading
				title="Derive Account"
				subtitle={parentPath}
				hasSubtitleIcon={true}
			/>
			<KeyboardScrollView extraHeight={Platform.OS === 'ios' ? 250 : 180}>
				{!isPathValid && <Text>Invalid Path</Text>}
				<TextInput
					autoCompleteType="off"
					autoCorrect={false}
					label="Path"
					onChangeText={setDerivationPath}
					onSubmitEditing={() => pathNameInput.current.focus()}
					placeholder="//hard/soft"
					returnKeyType="next"
					testID={testIDs.PathDerivation.pathInput}
					value={derivationPath}
				/>
				<TextInput
					autoCompleteType="off"
					autoCorrect={false}
					label="Display Name"
					onChangeText={keyParisName => setKeyPairsName(keyParisName)}
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
					identity={accounts.state.currentIdentity}
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
			</KeyboardScrollView>
			{isCustomNetwork && (
				<NetworkOptions
					setNetworkKey={setCustomNetworkKey}
					visible={modalVisible}
					setVisible={setModalVisible}
				/>
			)}
		</View>
	);
}

const styles = StyleSheet.create({
	container: {
		backgroundColor: colors.bg,
		flex: 1
	}
});

export default withAccountStore(withNavigation(PathDerivation));
