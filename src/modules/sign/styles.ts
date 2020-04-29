import {StyleSheet} from 'react-native';
import fontStyles from 'styles/fontStyles';

const styles = StyleSheet.create({
	title: {
		...fontStyles.h2,
		paddingBottom: 20
	},
	topTitle: {
		...fontStyles.h1,
		textAlign: 'center'
	},
	body: {
		paddingTop: 24
	},
	bodyContent: {
		marginVertical: 16,
		paddingHorizontal: 20
	},
	qr: {
		marginBottom: 20
	},
});

export default styles;
