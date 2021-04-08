import { showMessage as rnShowMessage } from 'react-native-flash-message';

export const showMessage = (message): void => {
	rnShowMessage({
		message,
		type: 'info'
	});
};
