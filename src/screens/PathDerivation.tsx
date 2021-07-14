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

import React, { useRef, useState, useEffect, useContext } from 'react';

import testIDs from 'e2e/testIDs';
import { AlertStateContext } from 'stores/alertContext';
import { NavigationAccountIdentityProps } from 'types/props';
import TextInput from 'components/TextInput';
import { alertPathDerivationError } from 'utils/alertUtils';
import Separator from 'components/Separator';
import ScreenHeading from 'components/ScreenHeading';
import Button from 'components/Button';
import { KeyboardAwareContainer } from 'components/Container';
import { tryCreateIdentity, suggestSeedName } from 'utils/native';
import { resetNavigationWithNetworkChooser } from 'utils/navigationHelpers';

export default function PathDerivation({
	navigation,
	route
}: NavigationAccountIdentityProps<'PathDerivation'>): React.ReactElement {
	const [derivationPath, setDerivationPath] = useState<string>(
		route.params.path
	);
	const [keyPairsName, setKeyPairsName] = useState<string>('');
	const [error, setError] = useState('');
	const pathNameInput = useRef<TextInput>(null);
	const { setAlert } = useContext(AlertStateContext);
	const networkKey = route.params.networkKey;

	//synchronize default name with path
	useEffect(() => {
		const updateSuggestedName = async (): Promise<void> => {
			const suggestion = await suggestSeedName(derivationPath);
			setKeyPairsName(suggestion);
		};
		updateSuggestedName();
	}, [derivationPath]);

	const onPathDerivation = async (): Promise<void> => {
		try {
			await tryCreateIdentity(
				keyPairsName,
				route.params.seedName,
				'sr25519',
				derivationPath,
				networkKey
			);
			setAlert('Success', 'New Account Successfully derived');
			resetNavigationWithNetworkChooser(navigation, 'PathsList', {
				networkKey
			});
		} catch (e) {
			setError(e.toString());
			alertPathDerivationError(setAlert, e.message);
		}
	};

	return (
		<KeyboardAwareContainer>
			<ScreenHeading title="Derive Account" error={!!error} subtitle={error} />
			<TextInput
				autoCompleteType="off"
				autoCorrect={false}
				autoFocus
				error={false}
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
			<Separator style={{ height: 0 }} />
			<Button
				disabled={false}
				title="Next"
				testID={testIDs.PathDerivation.deriveButton}
				onPress={onPathDerivation}
			/>
		</KeyboardAwareContainer>
	);
}
