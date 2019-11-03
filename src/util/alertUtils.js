import { Alert } from 'react-native';

export const alertIdentityCreationError = () =>
	Alert.alert('Error', "Can't create Identity from the seed", [
		{
			style: 'Cancel',
			text: 'Try again'
		}
	]);

export const alertPathDerivationError = () =>
	Alert.alert('Error', "Can't Derive Key pairs from the seed and paths", [
		{
			style: 'Cancel',
			text: 'Try again'
		}
	]);
