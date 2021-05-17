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

import React, { useState, useEffect } from 'react';
import { Text } from 'react-native';

import { MetadataCard } from 'modules/network/components/MetadataCard';
import Button from 'components/Button';
import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import { NavigationProps } from 'types/props';
import fontStyles from 'styles/fontStyles';
import { generateMetadataHandle } from 'utils/native';
import { saveMetadata } from 'utils/db';
import { MetadataHandle } from 'types/metadata';
//for tests
import testIDs from 'e2e/testIDs';

interface Props extends NavigationProps<'MetadataSaving'> {
	metadata: string;
}

export default function MetadataSaving({
	route,
	navigation
}: Props): React.ReactElement {
	const [metadataHandle, setMetadataHandle] = useState<MetadataHandle>();
	const { metadata } = route.params;

	useEffect(() => {
		const getHandle = async function (metadataString: string): Promise<void> {
			console.log(metadataString.substr(0, 128));
			const handle = await generateMetadataHandle(metadataString);
			setMetadataHandle(handle);
		};
		getHandle(metadata);
	}, [metadata]);

	const doSaveMetadata = async (): Promise<void> => {
		await saveMetadata(metadata);
		navigation.navigate('Main');
	};

	return (
		<SafeAreaViewContainer>
			<Text style={fontStyles.quote}>Metadata downloaded</Text>
			<MetadataCard
				specName={metadataHandle ? metadataHandle.specName : ''}
				specVersion={metadataHandle ? String(metadataHandle.specVersion) : ''}
				metadataHash={metadataHandle ? metadataHandle.hash : ''}
			/>
			<Button onPress={doSaveMetadata} title="Save metadata" />
		</SafeAreaViewContainer>
	);
}
