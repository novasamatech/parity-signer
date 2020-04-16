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

import { GenericExtrinsicPayload } from '@polkadot/types';
import React from 'react';
import { StyleSheet, Text, View } from 'react-native';

import Button from 'components/Button';
import CompatibleCard from 'components/CompatibleCard';
import PayloadDetailsCard from 'components/PayloadDetailsCard';
import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import ScreenHeading from 'components/ScreenHeading';
import TxDetailsCard from 'components/TxDetailsCard';
import { NETWORK_LIST } from 'constants/networkSpecs';
import testIDs from 'e2e/testIDs';
import AccountsStore from 'stores/AccountsStore';
import fontStyles from 'styles/fontStyles';
import { FoundAccount } from 'types/identityTypes';
import { isEthereumNetworkParams } from 'types/networkSpecsTypes';
import { NavigationAccountScannerProps } from 'types/props';
import { withAccountAndScannerStore } from 'utils/HOC';
import { getIdentityFromSender } from 'utils/identitiesUtils';
import {
	navigateToSignedTx,
	unlockSeedPhrase,
	unlockSeedPhraseWithPassword
} from 'utils/navigationHelpers';
import { useSeedRef } from 'utils/seedRefHooks';
import { constructSuriSuffix } from 'utils/suri';
import { Transaction } from 'utils/transaction';

function TxDetails({
	navigation,
	accounts,
	scannerStore
}: NavigationAccountScannerProps<'TxDetails'>): React.ReactElement {
	const txRequest = scannerStore.getTXRequest();
	const sender = scannerStore.getSender()!;
	const senderNetworkParams = NETWORK_LIST[sender.networkKey];
	const isEthereum = isEthereumNetworkParams(senderNetworkParams);
	const { isSeedRefValid, substrateSign, brainWalletSign } = useSeedRef(
		sender.encryptedSeed
	);

	async function onSignTx(): Promise<void> {
		try {
			if (sender.isLegacy) {
				navigation.navigate('AccountUnlockAndSign', {
					next: 'SignedTx'
				});
				return;
			}
			const senderIdentity = getIdentityFromSender(
				sender,
				accounts.state.identities
			);
			if (isEthereum) {
				await unlockSeedPhrase(navigation, isSeedRefValid);
				await scannerStore.signEthereumData(brainWalletSign);
			} else {
				let password = '';
				if (sender.hasPassword) {
					password = await unlockSeedPhraseWithPassword(
						navigation,
						isSeedRefValid,
						senderIdentity
					);
				}
				const suriSuffix = constructSuriSuffix({
					derivePath: sender.path,
					password
				});
				await scannerStore.signSubstrateData(substrateSign, suriSuffix);
			}
			return navigateToSignedTx(navigation);
		} catch (e) {
			scannerStore.setErrorMsg(e.message);
		}
	}

	if (txRequest) {
		const tx = scannerStore.getTx();
		return (
			<TxDetailsView
				{...(tx as Transaction)}
				accounts={accounts}
				isEthereum={isEthereum}
				sender={sender!}
				recipient={scannerStore.getRecipient()!}
				prehash={scannerStore.getPrehashPayload()!}
				onNext={onSignTx}
			/>
		);
	} else {
		return <View />;
	}
}

interface ViewProps extends Transaction {
	accounts: AccountsStore;
	gas: string;
	gasPrice: string;
	nonce: string;
	onNext: () => void;
	prehash: GenericExtrinsicPayload;
	recipient: FoundAccount;
	sender: FoundAccount;
	value: string;
	isEthereum: boolean;
}

function TxDetailsView({
	accounts,
	isEthereum,
	gas,
	gasPrice,
	onNext,
	prehash,
	recipient,
	sender,
	value
}: ViewProps): React.ReactElement {
	return (
		<SafeAreaScrollViewContainer
			style={styles.body}
			contentContainerStyle={{ paddingBottom: 120 }}
			testID={testIDs.TxDetails.scrollScreen}
		>
			<ScreenHeading
				title="Sign Transaction"
				subtitle="step 1/2 – verify and sign"
			/>
			<Text style={[fontStyles.t_big, styles.bodyContent]}>
				{`You are about to confirm sending the following ${
					isEthereum ? 'transaction' : 'extrinsic'
				}`}
			</Text>
			<View style={styles.bodyContent}>
				<CompatibleCard
					account={sender}
					accountsStore={accounts}
					titlePrefix={'from: '}
				/>
				{isEthereum ? (
					<View style={{ marginTop: 16 }}>
						<TxDetailsCard
							style={{ marginBottom: 20 }}
							description="You are about to send the following amount"
							value={value}
							gas={gas}
							gasPrice={gasPrice}
						/>
						<Text style={styles.title}>Recipient</Text>
						<CompatibleCard account={recipient} accountsStore={accounts} />
					</View>
				) : (
					<PayloadDetailsCard
						style={{ marginBottom: 20 }}
						payload={prehash}
						networkKey={sender.networkKey}
					/>
				)}
			</View>
			<View style={styles.signButtonContainer}>
				<Button
					buttonStyles={styles.signButton}
					testID={testIDs.TxDetails.signButton}
					title="Sign Transaction"
					onPress={(): any => onNext()}
				/>
			</View>
		</SafeAreaScrollViewContainer>
	);
}

export default withAccountAndScannerStore(TxDetails);

const styles = StyleSheet.create({
	body: {
		alignContent: 'flex-start'
	},
	bodyContent: {
		marginVertical: 16,
		paddingHorizontal: 20
	},
	marginBottom: {
		marginBottom: 16
	},
	signButton: {
		height: 60,
		paddingHorizontal: 60
	},
	signButtonContainer: {
		alignItems: 'center'
	},
	title: {
		...fontStyles.t_regular,
		paddingBottom: 8
	}
});
