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
import { isU8a, u8aToHex } from '@polkadot/util';
import React from 'react';
import { StyleSheet, Text, View } from 'react-native';
import { Subscribe } from 'unstated';

import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import testIDs from 'e2e/testIDs';
import { NETWORK_LIST } from 'constants/networkSpecs';
import { FoundAccount } from 'types/identityTypes';
import { isEthereumNetworkParams } from 'types/networkSpecsTypes';
import { NavigationProps } from 'types/props';
import colors from 'styles/colors';
import Button from 'components/Button';
import PayloadDetailsCard from 'components/PayloadDetailsCard';
import ScannerStore from 'stores/ScannerStore';
import AccountsStore from 'stores/AccountsStore';
import {
	navigateToSignedMessage,
	unlockSeedPhrase
} from 'utils/navigationHelpers';
import fontStyles from 'styles/fontStyles';
import MessageDetailsCard from 'components/MessageDetailsCard';
import { alertMultipart } from 'utils/alertUtils';
import CompatibleCard from 'components/CompatibleCard';
import { getIdentityFromSender } from 'utils/identitiesUtils';

export default class MessageDetails extends React.PureComponent<
	NavigationProps<'MessageDetails'>
> {
	async onSignMessage(
		scannerStore: ScannerStore,
		accountsStore: AccountsStore,
		sender: FoundAccount
	): Promise<void> {
		try {
			if (sender.isLegacy) {
				this.props.navigation.navigate('AccountUnlockAndSign', {
					next: 'SignedMessage'
				});
				return;
			}
			const senderIdentity = getIdentityFromSender(
				sender,
				accountsStore.state.identities
			);
			const seedPhrase = await unlockSeedPhrase(
				this.props.navigation,
				senderIdentity
			);
			await scannerStore.signDataWithSeedPhrase(
				seedPhrase,
				NETWORK_LIST[sender.networkKey].protocol
			);
			return navigateToSignedMessage(this.props.navigation);
		} catch (e) {
			scannerStore.setErrorMsg(e.message);
		}
	}

	render(): React.ReactElement {
		return (
			<Subscribe to={[ScannerStore, AccountsStore]}>
				{(
					scannerStore: ScannerStore,
					accountsStore: AccountsStore
				): React.ReactNode => {
					const dataToSign = scannerStore.getDataToSign()!;
					const message = scannerStore.getMessage()!;
					const sender = scannerStore.getSender()!;
					if (dataToSign) {
						return (
							<MessageDetailsView
								{...this.props}
								scannerStore={scannerStore}
								accountsStore={accountsStore}
								sender={sender}
								message={isU8a(message) ? u8aToHex(message) : message}
								dataToSign={
									//dataToSign could be U8A?
									isU8a(dataToSign)
										? u8aToHex(dataToSign)
										: dataToSign.toString()
								}
								prehash={scannerStore.getPrehashPayload()}
								isHash={scannerStore.getIsHash()}
								onNext={(): Promise<void> =>
									this.onSignMessage(scannerStore, accountsStore, sender)
								}
							/>
						);
					} else {
						return null;
					}
				}}
			</Subscribe>
		);
	}
}

interface Props extends NavigationProps<'MessageDetails'> {
	dataToSign: string;
	isHash?: boolean;
	message: string;
	onNext: () => void;
	prehash: GenericExtrinsicPayload | null;
	sender: FoundAccount;
	scannerStore: ScannerStore;
	accountsStore: AccountsStore;
}

export class MessageDetailsView extends React.PureComponent<Props> {
	render(): React.ReactElement {
		const {
			accountsStore,
			dataToSign,
			isHash,
			message,
			onNext,
			prehash,
			sender
		} = this.props;

		const networkParams = NETWORK_LIST[sender.networkKey];
		const isEthereum = isEthereumNetworkParams(networkParams);

		return (
			<SafeAreaScrollViewContainer
				contentContainerStyle={styles.bodyContent}
				style={styles.body}
				testID={testIDs.MessageDetails.scrollScreen}
			>
				<Text style={styles.topTitle}>Sign Message</Text>
				<Text style={styles.title}>From Account</Text>
				<CompatibleCard account={sender} accountsStore={accountsStore} />
				{!isEthereum && prehash ? (
					<PayloadDetailsCard
						description="You are about to confirm sending the following extrinsic. We will sign the hash of the payload as it is oversized."
						payload={prehash}
						networkKey={sender.networkKey}
					/>
				) : null}
				<MessageDetailsCard
					isHash={isHash ?? false}
					message={message}
					data={dataToSign}
				/>
				<View style={styles.signButtonContainer}>
					<Button
						buttonStyles={styles.signButton}
						testID={testIDs.MessageDetails.signButton}
						title="Sign Message"
						onPress={(): void => {
							isHash ? alertMultipart(onNext) : onNext();
						}}
					/>
				</View>
			</SafeAreaScrollViewContainer>
		);
	}
}

const styles = StyleSheet.create({
	actionButtonContainer: {
		flex: 1
	},
	actionsContainer: {
		flex: 1,
		flexDirection: 'row'
	},
	address: {
		flex: 1
	},
	body: {
		padding: 20
	},
	bodyContent: {
		paddingBottom: 40
	},
	changePinText: {
		color: 'green',
		textAlign: 'left'
	},
	deleteText: {
		textAlign: 'right'
	},
	signButton: {
		height: 60,
		paddingHorizontal: 60
	},
	signButtonContainer: {
		alignItems: 'center'
	},
	title: {
		...fontStyles.h2,
		paddingBottom: 20
	},
	topTitle: {
		...fontStyles.h1,
		paddingBottom: 20,
		textAlign: 'center'
	},
	transactionDetails: {
		backgroundColor: colors.card_bg,
		flex: 1
	},
	wrapper: {
		borderRadius: 5
	}
});
