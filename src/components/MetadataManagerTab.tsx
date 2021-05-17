import React from 'react';
import { StyleSheet, Text, View } from 'react-native';

import TouchableItem from './TouchableItem';
import Separator from './Separator';

import FastQrScannerTab from 'components/FastQrScannerTab';
//import SwitchShowAllMetadata from 'components/SwitchShowAllMetadata';
//import SwitchMetadataDeletion from 'components/SwitchMetadataDeletion';
import colors from 'styles/colors';
import fontStyles from 'styles/fontStyles';
import { ButtonListener } from 'types/props';
import testIDs from 'e2e/testIDs';

type MetadataManagerTabProps = {
	deletion: ButtonListener;
	showall: ButtonListener;
	isDeletion: boolean;
};

export default function MetadataManagerTab({
	deletion,
	showall,
	isDeletion
}: MetadataManagerTabProps): React.ReactElement {
	return (
		<View style={styles.body}>
			<Separator
				shadow={true}
				style={{ backgroundColor: 'transparent', marginVertical: 0 }}
				shadowStyle={{ height: 16, marginTop: -16 }}
			/>
			<View style={styles.tab}>
				<TouchableItem
					accessibilityComponentType="button"
					onPress={deletion}
					style={styles.derivationButton}
					testID={testIDs.MetadataManagement.deleteMetadataSwitch}
				>
					<Text style={isDeletion ? styles.activeIcon : styles.icon}>🗑</Text>
					<Text style={isDeletion ? styles.textLabelActive : styles.textLabel}>
						Delete metadata mode
					</Text>
				</TouchableItem>
			</View>
			<View style={styles.tab}>
				<FastQrScannerTab />
			</View>
			<View style={styles.tab}>
				<TouchableItem
					accessibilityComponentType="button"
					onPress={showall}
					style={styles.derivationButton}
				>
					<Text style={styles.icon}>⇕</Text>
					<Text style={styles.textLabel}>Show all metadata</Text>
				</TouchableItem>
			</View>
		</View>
	);
}

const styles = StyleSheet.create({
	activeIcon: {
		...fontStyles.i_large,
		color: colors.signal.error,
		fontWeight: 'bold',
		marginTop: 8
	},
	body: { flexDirection: 'row' },
	derivationButton: {
		alignItems: 'center',
		backgroundColor: colors.background.os,
		borderBottomColor: colors.background.app,
		borderBottomWidth: 1,
		height: 72,
		justifyContent: 'center',
		paddingVertical: 9
	},
	icon: {
		...fontStyles.i_large,
		color: colors.signal.main,
		fontWeight: 'bold',
		marginTop: 8
	},
	tab: {
		flex: 1,
		flexGrow: 1
	},
	textLabel: {
		...fontStyles.a_text,
		color: colors.text.faded,
		marginTop: 4
	},
	textLabelActive: {
		...fontStyles.a_text,
		color: colors.text.alert,
		marginTop: 4
	}
});
