import { StyleSheet } from 'react-native';
import fontStyles from 'styles/fontStyles';

const styles = StyleSheet.create({
	body: { paddingTop: 24 },
	bodyContent: {
		marginVertical: 16,
		paddingHorizontal: 20
	},
	qr: { marginBottom: 8 },
	title: {
		...fontStyles.h2,
		paddingBottom: 20
	},
	topTitle: {
		...fontStyles.h1,
		textAlign: 'center'
	}
});

export default styles;
