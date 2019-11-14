import React from 'react';
import { withAccountStore } from '../util/HOC';
import { withNavigation } from 'react-navigation';
import { ScrollView } from 'react-native';
import TextInput from '../components/TextInput';
import PathCard from '../components/PathCard';
import colors from '../colors';

function PathManagement({ accounts, navigation }) {
	const path = navigation.getParam('path', '');
	const { currentIdentity } = accounts.state;
	const pathName = currentIdentity.meta.get(path).name;

	return (
		<ScrollView style={{ backgroundColor: colors.bg }}>
			<PathCard identity={currentIdentity} path={path} />
			<TextInput
				label="Display Name"
				onChangeText={name => accounts.updatePathName(path, name)}
				value={pathName}
				placeholder="Enter a new identity name"
				focus={true}
			/>
		</ScrollView>
	);
}

export default withAccountStore(withNavigation(PathManagement));
