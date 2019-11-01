import { NavigationActions, StackActions } from 'react-navigation';

export const setPin = async navigation =>
	new Promise(resolve => {
		navigation.navigate('IdentityPin', { isNew: true, resolve });
	});

export const unlockSeed = async navigation =>
	new Promise(resolve => {
		navigation.navigate('IdentityPin', { isUnlock: true, resolve });
	});

export const resetToPathsList = (navigation, networkKey) => {
	const resetAction = StackActions.reset({
		actions: [
			NavigationActions.navigate({
				isNew: false,
				routeName: 'AccountNetworkChooser'
			}),
			NavigationActions.navigate({
				params: { networkKey },
				routeName: 'PathsList'
			})
		],
		index: 1,
		key: undefined
	});
	navigation.dispatch(resetAction);
};
