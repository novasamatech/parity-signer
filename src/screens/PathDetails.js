import React from 'react';
import { withAccountStore } from '../util/HOC';
import { withNavigation } from 'react-navigation';
import { ScrollView, StyleSheet, Text, View } from 'react-native';
import PathCard from '../components/PathCard';
import PopupMenu from '../components/PopupMenu';
import colors from '../colors';
import fonts from '../fonts';
import QrView from '../components/QrView';
import { getNetworkKeyByPath } from '../util/identitiesUtils';
import { accountId } from '../util/account';
import { UnknownNetworkKeys } from '../constants';
import { alertDeleteAccount } from '../util/alertUtils';
import { navigateToPathsList } from '../util/navigationHelpers';

function PathDetails({ accounts, navigation }) {
	const { currentIdentity } = accounts.state;
	const path = navigation.getParam('path', '');
	const networkKey = getNetworkKeyByPath(path);
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
				<QrView data={accountId({ address, networkKey })} />
			)}
		</ScrollView>
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
