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

import React, { ReactElement } from 'react';
import {
	FlatList,
	StyleSheet,
	Text,
	TouchableOpacity,
	View
} from 'react-native';
import Icon from 'react-native-vector-icons/MaterialIcons';

import AccountIcon from './AccountIcon';
import Address from './Address';

import colors from 'styles/colors';
import { NetworkProtocols } from 'constants/networkSpecs';
import fonts from 'styles/fonts';
import { debounce } from 'utils/debounce';
import { brainWalletAddress, substrateAddress, words } from 'utils/native';
import { constructSURI } from 'utils/suri';
import { NetworkParams, SubstrateNetworkParams } from 'types/networkSpecsTypes';

interface IconType {
	address: string;
	bip39: boolean;
	seed: string;
}

interface Props {
	derivationPassword: string;
	derivationPath: string;
	network: NetworkParams;
	onSelect: (icon: {
		isBip39: boolean;
		newAddress: string;
		newSeed: string;
	}) => void;
	value: string;
}

export default class AccountIconChooser extends React.PureComponent<
	Props,
	{ icons: IconType[] }
> {
	constructor(props: any) {
		super(props);

		this.state = {
			icons: []
		};
	}

	refreshIcons = async (): Promise<void> => {
		const {
			derivationPassword,
			derivationPath,
			network,
			onSelect
		} = this.props;

		// clean previous selection
		onSelect({ isBip39: false, newAddress: '', newSeed: '' });
		try {
			const icons = await Promise.all(
				Array(4)
					.join(' ')
					.split(' ')
					.map(async () => {
						const result = {
							address: '',
							bip39: false,
							seed: ''
						};
						result.seed = await words(24);

						if (network.protocol === NetworkProtocols.ETHEREUM) {
							Object.assign(result, await brainWalletAddress(result.seed));
						} else {
							// Substrate
							try {
								const suri = constructSURI({
									derivePath: derivationPath,
									password: derivationPassword,
									phrase: result.seed
								});

								result.address = await substrateAddress(
									suri,
									(network as SubstrateNetworkParams).prefix
								);
								result.bip39 = true;
							} catch (e) {
								// invalid seed or derivation path
								console.error(e);
							}
						}
						return result;
					})
			);
			this.setState({ icons });
		} catch (e) {
			console.error(e);
		}
	};

	renderAddress = (): ReactElement => {
		const {
			network: { protocol },
			value
		} = this.props;

		if (value) {
			return (
				<Address
					address={value}
					protocol={protocol}
					style={styles.addressText}
				/>
			);
		} else {
			return <Text style={styles.addressSelectionText}>Select an icon.</Text>;
		}
	};

	renderIcon = ({
		item,
		index
	}: {
		item: IconType;
		index: number;
	}): ReactElement => {
		const { onSelect, network, value } = this.props;
		const { address, bip39, seed } = item;
		const isSelected = address.toLowerCase() === value.toLowerCase();

		if (!address) {
			//return an empty view to prevent the screen from jumping
			return <View style={styles.icon} />;
		}

		return (
			<TouchableOpacity
				key={index}
				style={[styles.iconBorder, isSelected ? styles.selected : {}]}
				onPress={(): void =>
					onSelect({ isBip39: bip39, newAddress: address, newSeed: seed })
				}
			>
				<AccountIcon address={address} network={network} style={styles.icon} />
			</TouchableOpacity>
		);
	};

	componentDidMount(): void {
		this.refreshIcons();
	}

	debouncedRefreshIcons = debounce(this.refreshIcons, 200);

	componentDidUpdate(prevProps: any): void {
		const { derivationPassword, derivationPath, network } = this.props;

		if (
			prevProps.network !== network ||
			prevProps.derivationPassword !== derivationPassword ||
			prevProps.derivationPath !== derivationPath
		) {
			this.debouncedRefreshIcons();
		}
	}

	render(): React.ReactElement {
		const { value } = this.props;
		const { icons } = this.state;

		return (
			<View style={styles.body}>
				<View style={styles.firstRow}>
					<FlatList
						data={icons}
						extraData={value}
						horizontal
						keyExtractor={(item: IconType): string => item.seed}
						renderItem={this.renderIcon}
					/>
					<TouchableOpacity onPress={this.refreshIcons}>
						<Icon name={'refresh'} size={35} style={styles.refreshIcon} />
					</TouchableOpacity>
				</View>
				{this.renderAddress()}
			</View>
		);
	}
}

const styles = StyleSheet.create({
	addressSelectionText: {
		color: colors.bg,
		fontFamily: fonts.regular,
		fontSize: 14,
		lineHeight: 16,
		paddingLeft: 6
	},
	addressText: {
		fontSize: 14,
		paddingLeft: 6
	},
	body: {
		backgroundColor: colors.card_bg,
		display: 'flex',
		flexDirection: 'column',
		marginBottom: 20,
		padding: 20
	},
	firstRow: {
		alignItems: 'center',
		display: 'flex',
		flex: 1,
		flexDirection: 'row',
		marginBottom: 10
	},
	icon: {
		backgroundColor: colors.card_bg,
		height: 50,
		padding: 5,
		width: 50
	},
	iconBorder: {
		borderColor: colors.card_bg,
		borderWidth: 6,
		height: 62 // height = icon height + borderWidth * 2
	},
	refreshIcon: {
		color: colors.bg
	},
	selected: {
		borderColor: colors.bg
	}
});
