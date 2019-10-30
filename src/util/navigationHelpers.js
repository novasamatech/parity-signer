export const setPin = async navigation =>
	new Promise(resolve => {
		navigation.navigate('IdentityPin', { isNew: true, resolve });
	});
