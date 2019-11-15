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

import React, { useState } from 'react';
import { withNavigation } from 'react-navigation';
import { withAccountStore } from '../util/HOC';
import { StyleSheet, Text, View } from 'react-native';
import TextInput from '../components/TextInput';
import ButtonMainAction from '../components/ButtonMainAction';
import { validateDerivedPath } from '../util/identitiesUtils';
import { navigateToPathsList, unlockSeed } from '../util/navigationHelpers';
import { NETWORK_LIST, UnknownNetworkKeys } from '../constants';
import { alertPathDerivationError } from '../util/alertUtils';
import testIDs from '../../e2e/testIDs';
import Separator from '../components/Separator';
import ScreenHeading from '../components/ScreenHeading';
import colors from '../colors';
import PathCard from '../components/PathCard';
import KeyboardScrollView from '../components/KeyboardScrollView';

function PathDerivation({ accounts, navigation }) {
	const networkKey = navigation.getParam(
		'networkKey',
		UnknownNetworkKeys.UNKNOWN
	);

	const [derivationPath, setDerivationPath] = useState('');
	const [keyPairsName, setKeyPairsName] = useState('');
	const [isPathValid, setIsPathValid] = useState(true);
	const existedNetworkPath = `//${NETWORK_LIST[networkKey].pathId}`;
	const completePath = `${existedNetworkPath}${derivationPath}`;

	return (
		<View style={styles.container}>
			<ScreenHeading
				title="Derive Account"
				subtitle={existedNetworkPath}
				subtitleIcon={true}
			/>
			<KeyboardScrollView>
				{!isPathValid && <Text>Invalid Path</Text>}
				<TextInput
					label="Path"
					placeholder="//hard/soft"
					autoFocus
					value={derivationPath}
					testID={testIDs.PathDerivation.pathInput}
					onChangeText={setDerivationPath}
				/>
				<TextInput
					label="Display Name"
					testID={testIDs.PathDerivation.nameInput}
					value={keyPairsName}
					onChangeText={keyParisName => setKeyPairsName(keyParisName)}
				/>
				<Separator style={{ height: 0 }} />
				<PathCard
					identity={accounts.state.currentIdentity}
					path={completePath}
				/>
				<ButtonMainAction
					disabled={!validateDerivedPath(derivationPath)}
					bottom={false}
					style={{ marginTop: 8 }}
					title="Derive Address"
					testID={testIDs.PathDerivation.deriveButton}
					onPress={async () => {
						if (!validateDerivedPath(derivationPath)) {
							return setIsPathValid(false);
						}
						const seed = await unlockSeed(navigation);
						const derivationSucceed = await accounts.deriveNewPath(
							completePath,
							seed,
							NETWORK_LIST[networkKey].prefix,
							networkKey
						);
						if (derivationSucceed) {
							navigateToPathsList(navigation, networkKey);
						} else {
							setIsPathValid(false);
							alertPathDerivationError();
						}
					}}
				/>
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

export default withAccountStore(withNavigation(PathDerivation));
