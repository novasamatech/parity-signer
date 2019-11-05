import React from 'react';
import PropTypes from 'prop-types';
import { Text, View } from 'react-native';
import Button from './Button';
import { getPathName, isSubstratePath } from '../util/identitiesUtils';

PathCard.propTypes = {
	identity: PropTypes.object.isRequired,
	onPress: PropTypes.func,
	path: PropTypes.string.isRequired
};

export default function PathCard({ onPress, identity, path }) {
	const pathName = getPathName(path, identity);
	const address = identity.meta.get(path).address;

	return (
		<View>
			<Text>{pathName}</Text>
			{isSubstratePath(path) && <Button onPress={onPress} title={path} />}
			<Text>{address}</Text>
		</View>
	);
}
