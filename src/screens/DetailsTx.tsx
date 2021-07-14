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

// This screen shows payload details and asks for signing confirmation

import React, { useContext, useEffect, useState } from 'react';
import { Text, View, FlatList, StyleSheet } from 'react-native';
import fontStyles from 'styles/fontStyles';

import { PayloadCardData, Action } from 'types/payloads';
import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import testIDs from 'e2e/testIDs';
import { NavigationProps } from 'types/props';
import Button from 'components/Button';
import { makeTransactionCardsContents, accept } from 'utils/native';
import PayloadCard from 'components/PayloadCard';
import { AlertStateContext } from 'stores/alertContext';
import { resetNavigationWithNetworkChooser } from 'utils/navigationHelpers';

function DetailsTx({
	route,
	navigation
}: NavigationProps<'DetailsTx'>): React.ReactElement {
	const payload = route.params ? route.params.payload : '';
	const [payloadCards, setPayloadCards] = useState<PayloadCardData[]>([
		{ indent: 0, index: 0, payload: {}, type: 'loading' }
	]);
	const { setAlert } = useContext(AlertStateContext);
	const [author, setAuthor] = useState();
	const [action, setAction] = useState<Action>({
		payload: '',
		type: ''
	});

	useEffect(() => {
		const generateCards = async function (encoded: string): Promise<void> {
			const cardsSet = await makeTransactionCardsContents(encoded);
			if (cardsSet.author) setAuthor(cardsSet.author[0].payload);
			//TODO: here should be finer features on what to do
			//with different payload types.
			//
			//last sort seems useless but things depend
			//on undocumented features otherwise
			const sortedCardSet = []
				.concat(
					cardsSet.author ? cardsSet.author : [],
					cardsSet.verifier ? cardsSet.verifier : [],
					cardsSet.error ? cardsSet.error : [],
					cardsSet.warning ? cardsSet.warning : [],
					cardsSet.method ? cardsSet.method : [],
					cardsSet.meta ? cardsSet.meta : [],
					cardsSet.extrinsics ? cardsSet.extrinsics : []
				)
				.sort((a, b) => {
					return a.index - b.index;
				});
			console.log(sortedCardSet);
			setPayloadCards(
				sortedCardSet
					? sortedCardSet
					: [{
							indent: 0,
							index: 0,
							payload: 'System error: transaction parser failed entirely',
							type: 'error'
					  }]
			);
			if (cardsSet.action) setAction(cardsSet.action);
		};
		if (
			payload ===
			'5301025a4a03f84a19cf8ebda40e62358c592870691a9cf456138bb4829969d10fe969a40403005a4a03f84a19cf8ebda40e62358c592870691a9cf456138bb4829969d10fe9690700e40b5402c5005c00ec07000004000000b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafebcd1b489599db4424ed928804ddad3a4689fb8c835151ef34bc250bb845cdc1eb0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe'
		) {
			generateCards('test all');
		} else {
			generateCards(payload);
		}
	}, [payload]);

	const renderCard = ({
		item
	}: {
		item: PayloadCardData;
	}): React.ReactElement => {
		return (
			<View style={[{ paddingLeft: item.indent * 4 + '%' }]}>
				<PayloadCard type={item.type} payload={item.payload} />
			</View>
		);
	};

	const performAction = async (): void => {
		console.log(action);
		if (action.type === 'sign_transaction') {
			resetNavigationWithNetworkChooser(navigation, 'SignedTx', { payload: action.payload, author: author });
		} else if (action.type) {
			const acceptResult = await accept(JSON.stringify(action.payload));
			setAlert('Accept result', acceptResult);
			navigation.goBack();
		} else {
			navigation.goBack();
		}
		return;
	};

	return (
		<SafeAreaViewContainer testID={testIDs.DetailsTx.detailsScreen}>
			<Text style={styles.topTitle}>Payload</Text>
			<FlatList
				data={payloadCards}
				renderItem={renderCard}
				keyExtractor={(item: PayloadCardData): number => item.index.toString()}
			/>
			<Button
				onPress={performAction}
				title={
					action.type === 'sign_transaction'
						? 'SIGN'
						: action.type
						? 'ACCEPT'
						: 'BACK'
				}
				testID={testIDs.DetailsTx.signButton}
			/>
		</SafeAreaViewContainer>
	);
}

export default DetailsTx;

const styles = StyleSheet.create({
	body: {
		paddingTop: 24
	},
	bodyContent: {
		marginVertical: 16,
		paddingHorizontal: 20
	},
	qr: {
		marginBottom: 8
	},
	title: {
		...fontStyles.h2,
		paddingBottom: 20
	},
	topTitle: {
		...fontStyles.h1,
		textAlign: 'center'
	}
});

