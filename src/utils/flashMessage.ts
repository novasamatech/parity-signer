import { showMessage as rnShowMessage } from 'react-native-flash-message';

export const showMessage = (message: string): void => {
	rnShowMessage({
		message,
		type: 'info'
	});
};
