import React from 'react';
import PropTypes from 'prop-types';
import { Text, View } from 'react-native';
import Button from './Button';
import { getPathName } from '../util/identitiesUtils';

PathCard.propTypes = {
	identity: PropTypes.object.isRequired,
	onPress: PropTypes.func,
	path: PropTypes.string.isRequired
};

export default function PathCard({ onPress, identity, path }) {
	const pathName = getPathName(path, identity);

	return (
		<View>
			<Text>{pathName}</Text>
			<Button onPress={onPress} title={path} />
		</View>
	);
}
