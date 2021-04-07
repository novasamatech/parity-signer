import { showMessage } from 'react-native-flash-message';

export const showMessage = (message) => {
  showMessage({
    message,
    type: 'info',
  });
};
