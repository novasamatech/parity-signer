import React from 'react';
import { withAccountStore } from '../util/HOC';
import { withNavigation } from 'react-navigation';
import { ScrollView, StyleSheet, View } from 'react-native';
import PathCard from '../components/PathCard';
import PopupMenu from '../components/PopupMenu';
import ScreenHeading from '../components/ScreenHeading';
import colors from '../colors';
import QrView from '../components/QrView';
import {
	getAddressWithPath,
	getNetworkKeyByPath,
	isSubstratePath
} from '../util/identitiesUtils';
import { UnknownNetworkKeys } from '../constants';
import { alertDeleteAccount, alertPathDeletionError } from '../util/alertUtils';
import { navigateToPathsList } from '../util/navigationHelpers';
import testIDs from '../../e2e/testIDs';

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
			<ScreenHeading small={true} title="Public Address" />
			<View style={styles.menuView}>
				<PopupMenu
					testID={testIDs.PathDetail.popupMenuButton}
					onSelect={onOptionSelect}
					menuTriggerIconName={'more-vert'}
					menuItems={[
						{ text: 'Edit', value: 'PathManagement' },
						{
							testID: testIDs.PathDetail.deleteButton,
							text: 'Delete',
							textStyle: styles.deleteText,
							value: 'PathDelete'
						}
					]}
				/>
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
	const networkKey = getNetworkKeyByPath(path);
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
	deleteText: {
		color: colors.bg_alert
	},
	menuView: {
		alignItems: 'flex-end',
		flex: 1,
		position: 'absolute',
		right: 16,
		top: 5
	}
});

export default withAccountStore(withNavigation(PathDetails));
