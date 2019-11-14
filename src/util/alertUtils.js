import { Alert } from 'react-native';

export const alertIdentityCreationError = () =>
	Alert.alert('Error', "Can't create Identity from the seed", [
		{
			style: 'Cancel',
			text: 'Try again'
		}
	]);

export const alertPathDerivationError = () =>
	Alert.alert('Error', "Can't Derive Key pairs from the seed", [
		{
			style: 'Cancel',
			text: 'Try again'
		}
	]);

export const alertPathDeletionError = () =>
	Alert.alert('Error', "Can't delete Key pairs.", [
		{
			style: 'Cancel',
			text: 'Try again'
		}
	]);

export const alertDeleteAccount = (accountName, onDelete) => {
	Alert.alert(
		'Delete Key Pairs',
		`Do you really want to delete ${accountName}?
This account can only be recovered with its associated recovery phrase.`,
		[
			{
				onPress: () => {
					onDelete();
				},
				style: 'destructive',
				text: 'Delete'
			},
			{
				style: 'cancel',
				text: 'Cancel'
			}
		]
	);
};
