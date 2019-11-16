import { NavigationActions, StackActions } from 'react-navigation';

export const setPin = async navigation =>
	new Promise(resolve => {
		navigation.navigate('IdentityPin', { isNew: true, resolve });
	});

export const unlockSeed = async navigation =>
	new Promise(resolve => {
		navigation.navigate('IdentityPin', { isUnlock: true, resolve });
	});

export const navigateToPathsList = (navigation, networkKey) => {
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

export const navigateToLandingPage = (navigation, isSwitchOpen) => {
	const resetAction = StackActions.reset({
		actions: [
			NavigationActions.navigate({
				params: { isSwitchOpen },
				routeName: 'AccountNetworkChooser'
			})
		],
		index: 0,
		key: undefined
	});
	navigation.dispatch(resetAction);
};

export const navigateToNewIdentityNetwork = navigation => {
	const resetAction = StackActions.reset({
		actions: [
			NavigationActions.navigate({
				params: { isNew: true },
				routeName: 'AccountNetworkChooser'
			})
		],
		index: 0,
		key: undefined
	});
	navigation.dispatch(resetAction);
};

export const navigateToSignedMessage = navigation => {
	const resetAction = StackActions.reset({
		actions: [
			NavigationActions.navigate({
				isNew: false,
				routeName: 'AccountNetworkChooser'
			}),
			NavigationActions.navigate({
				params: { isNew: true },
				routeName: 'SignedMessage'
			})
		],
		index: 1,
		key: undefined
	});
	navigation.dispatch(resetAction);
};

export const navigateToSignedTx = navigation => {
	const resetAction = StackActions.reset({
		actions: [
			NavigationActions.navigate({
				isNew: false,
				routeName: 'AccountNetworkChooser'
			}),
			NavigationActions.navigate({
				params: { isNew: true },
				routeName: 'SignedTx'
			})
		],
		index: 1,
		key: undefined
	});
	navigation.dispatch(resetAction);
};

export const navigateToLegacyAccountList = navigation => {
	const resetAction = StackActions.reset({
		actions: [NavigationActions.navigate({ routeName: 'LegacyAccountList' })],
		index: 0, // FIXME workaround for now, use SwitchNavigator later: https://github.com/react-navigation/react-navigation/issues/1127#issuecomment-295841343
		key: undefined
	});
	navigation.dispatch(resetAction);
};
