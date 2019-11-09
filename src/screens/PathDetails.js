import React from 'react';
import { withAccountStore } from '../util/HOC';
import { withNavigation } from 'react-navigation';
import { ScrollView, StyleSheet, Text, View } from 'react-native';
import PathCard from '../components/PathCard';
import PopupMenu from '../components/PopupMenu';
import colors from '../colors';
import fonts from '../fonts';
import QrView from '../components/QrView';
import { getNetworkKeyBySubstratePath } from '../util/identitiesUtils';
import { UnknownNetworkKeys } from '../constants';
import { alertDeleteAccount } from '../util/alertUtils';
import { navigateToPathsList } from '../util/navigationHelpers';
import Button from '../components/Button';

export function PathDetailsView({ accounts, navigation, path, networkKey }) {
	const { currentIdentity } = accounts.state;
	const { address } = currentIdentity.meta.get(path) || {};

	const onOptionSelect = value => {
		if (value === 'PathDelete') {
			alertDeleteAccount('this key pairs', async () => {
				const deleteSucceed = await accounts.deletePath(path);
				if (deleteSucceed) navigateToPathsList(navigation, networkKey);
			});
		} else {
			navigation.navigate('PathManagement', { path });
		}
	};

	return (
		<ScrollView>
			<PathCard identity={currentIdentity} path={path} />
			<View style={styles.header}>
				<Text style={styles.title}>PUBLIC ADDRESS</Text>
				<View style={styles.menuView}>
					<PopupMenu
						onSelect={onOptionSelect}
						menuTriggerIconName={'more-vert'}
						menuItems={[
							{ text: 'Edit', value: 'PathManagement' },
							{
								text: 'Delete',
								textStyle: styles.deleteText,
								value: 'PathDelete'
							}
						]}
					/>
				</View>
			</View>
			{networkKey !== UnknownNetworkKeys.UNKNOWN && address && (
				<QrView data={address} />
			)}
			<Button title="Scan" onPress={() => navigation.navigate('QrScanner')} />
		</ScrollView>
	);
}

function PathDetails({ accounts, navigation }) {
	const path = navigation.getParam('path', '');
	const networkKey = getNetworkKeyBySubstratePath(path);
	return (
		<PathDetailsView
			accounts={accounts}
			navigation={navigation}
			path={path}
			networkKey={networkKey}
		/>
	);
}

const styles = StyleSheet.create({
	header: {
		alignItems: 'center',
		flexDirection: 'row',
		justifyContent: 'center'
	},
	menuView: {
		alignItems: 'flex-end',
		flex: 1
	},
	qr: {
		backgroundColor: colors.card_bg,
		marginTop: 20
	},
	title: {
		color: colors.bg_text_sec,
		flexDirection: 'column',
		fontFamily: fonts.bold,
		fontSize: 18,
		justifyContent: 'center'
	}
});

export default withAccountStore(withNavigation(PathDetails));
