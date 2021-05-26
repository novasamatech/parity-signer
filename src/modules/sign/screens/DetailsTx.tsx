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

// This screen shows Tx-type payload details and asks for signing confirmation

import React, { useContext, useEffect, useRef, useState } from 'react';
import { Text, View, FlatList } from 'react-native';

import strings from 'modules/sign/strings';
import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import testIDs from 'e2e/testIDs';
import { AccountsContext } from 'stores/AccountsContext';
import { NetworksContext } from 'stores/NetworkContext';
import { ScannerContext } from 'stores/ScannerContext';
import { FoundAccount } from 'types/identityTypes';
import { isEthereumNetworkParams } from 'types/networkTypes';
import { PayloadCardData } from 'types/payload';
import { NavigationProps, NavigationScannerProps } from 'types/props';
import CompatibleCard from 'components/CompatibleCard';
import { Transaction } from 'utils/transaction';
import styles from 'modules/sign/styles';
import Separator from 'components/Separator';
import Button from 'components/Button';
import { makeTransactionCardsContents } from 'utils/native';
import PayloadCard from 'modules/sign/components/PayloadCard';
import { getMetadata } from 'utils/db';
import { typeDefs } from 'constants/typeDefs';

function DetailsTx({
	route,
	navigation
}: NavigationProps<'DetailsTx'>): React.ReactElement {
	const scannerStore = useContext(ScannerContext);
	const { recipient, sender } = scannerStore.state;
	const cleanup = useRef(scannerStore.cleanup);

	useEffect(() => cleanup.current, [cleanup]);

	if (sender === null || recipient === null) return <View />;
	return (
		<UnsignedTxView
			sender={sender}
			recipient={recipient}
			scannerStore={scannerStore}
			route={route}
			navigation={navigation}
		/>
	);
}

interface Props extends NavigationScannerProps<'DetailsTx'> {
	sender: FoundAccount;
	recipient: FoundAccount;
}

function UnsignedTxView({
	sender,
	recipient,
	scannerStore,
	route
}: Props): React.ReactElement {
	const accountsStore = useContext(AccountsContext);
	const { idumpNetworksData, getNetwork } = useContext(NetworksContext);
	const { tx, rawPayload } = scannerStore.state;
	const senderNetworkParams = getNetwork(sender.networkKey);
	const isEthereum = isEthereumNetworkParams(senderNetworkParams);
	const { value, gas, gasPrice } = tx as Transaction;
	const payload = null;
	const [ payloadCards, setPayloadCards ] = useState<PayloadCardData[]>([])

	useEffect(() => {
		const generateCards = async function (encoded: string): Promise<void> {
			const networksData = dumpNetworksData();
			const metadata = await getMetadata(senderNetworkParams.metadata);
			const cardsSet = await makeTransactionCardsContents(encoded, networksData, metadata, typedefs);
			console.log(cardsSet.method.concat(cardsSet.extrinsics));
			setPayloadCards(cardsSet.method.concat(cardsSet.extrinsics));
		}
		generateCards(rawPayload);
	}, [rawPayload]);
	
	const renderCard = ({ item }: { item: PayloadCard }): ReactElement => {
		return (
			<View style={[{paddingLeft: item.indent*4 + '%'}]}>
				<PayloadCard 
					type={item.type}
					payload={item.payload}
				/>
			</View>
		);
	};

	const approveTransaction = (): void => {
		const resolve = route.params.resolve;
		resolve();
	};

	return (
		<SafeAreaViewContainer testID={testIDs.DetailsTx.detailsScreen}>
			<Text style={styles.topTitle}>Extrinsic to sign</Text>
			<CompatibleCard
				account={sender}
				accountsStore={accountsStore}
				titlePrefix={'from:'}
			/>
			<FlatList 
				data={payloadCards}
				renderItem={renderCard}
				keyExtractor={(item: PayloadCard): number => item.index}
			/>
			<Button
				onPress={approveTransaction}
				title="SIGN"
				testID={testIDs.DetailsTx.signButton}
			/>
		</SafeAreaViewContainer>
	);
}

export default DetailsTx;
