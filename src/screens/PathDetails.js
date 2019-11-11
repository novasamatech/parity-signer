import React from 'react';
import { withAccountStore } from '../util/HOC';
import { withNavigation } from 'react-navigation';
import { ScrollView, StyleSheet, Text, View } from 'react-native';
import PathCard from '../components/PathCard';
import PopupMenu from '../components/PopupMenu';
import colors from '../colors';
import fonts from '../fonts';
import QrView from '../components/QrView';
import {
	getAddressWithPath,
	getNetworkKeyBySubstratePath,
	isSubstratePath
} from '../util/identitiesUtils';
import { UnknownNetworkKeys } from '../constants';
import { alertDeleteAccount, alertPathDeletionError } from '../util/alertUtils';
import { navigateToPathsList } from '../util/navigationHelpers';

export function PathDetailsView({ accounts, navigation, path, networkKey }) {
	const { currentIdentity } = accounts.state;
	const address = getAddressWithPath(path, currentIdentity);

	const onOptionSelect = value => {
		if (value === 'PathDelete') {
			alertDeleteAccount('this key pairs', async () => {
				const deleteSucceed = await accounts.deletePath(path);
				if (deleteSucceed) {
					isSubstratePath(path)
						? navigateToPathsList(navigation, networkKey)
						: navigation.navigate('AccountNetworkChooser');
				} else {
					alertPathDeletionError();
				}
			});
		} else {
			navigation.navigate('PathManagement', { path });
		}
	};

	return (
		<ScrollView style={styles.body}>
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
			<PathCard identity={currentIdentity} path={path} />
			{networkKey !== UnknownNetworkKeys.UNKNOWN && address !== '' && (
				<QrView data={address} />
			)}
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
	body: {
		backgroundColor: colors.bg,
		flex: 1,
		flexDirection: 'column'
	},
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
