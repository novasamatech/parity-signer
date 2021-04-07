import { showMessage as rnShowMessage } from 'react-native-flash-message';

export const showMessage = message => {
	rnShowMessage({
		message,
		type: 'info'
	});
};
