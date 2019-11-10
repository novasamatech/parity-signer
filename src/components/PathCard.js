import React from 'react';
import PropTypes from 'prop-types';
import { Text, View } from 'react-native';
import Button from './Button';
import {
	getNetworkKeyBySubstratePath,
	getPathName,
	isSubstratePath
} from '../util/identitiesUtils';
import { NETWORK_LIST } from '../constants';

PathCard.propTypes = {
	identity: PropTypes.object.isRequired,
	onPress: PropTypes.func,
	path: PropTypes.string.isRequired,
	testID: PropTypes.string
};

export default function PathCard({ onPress, identity, path, testID }) {
	const pathName = getPathName(path, identity);
	const address = identity.meta.get(path).address;

	const networkKey = getNetworkKeyBySubstratePath(path);
	const networkParams = NETWORK_LIST[networkKey];

	return (
		<View testID={testID}>
			<Text>{networkParams.color.toString()}</Text>
			<Text>{pathName}</Text>
			{isSubstratePath(path) && <Button onPress={onPress} title={path} />}
			<Text>{address}</Text>
		</View>
	);
}
