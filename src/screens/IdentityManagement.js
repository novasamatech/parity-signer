import React from 'react';
import { withAccountStore } from '../util/HOC';
import { withNavigation } from 'react-navigation';
import { ScrollView, Text } from 'react-native';

function IdentityManagement() {
	return (
		<ScrollView>
			<Text>Display Name</Text>
		</ScrollView>
	);
}

export default withAccountStore(withNavigation(IdentityManagement));
