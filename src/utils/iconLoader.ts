import AntIcon from 'react-native-vector-icons/AntDesign';
import FeatherIcon from 'react-native-vector-icons/Feather';
import FontAwesome from 'react-native-vector-icons/FontAwesome';
import MaterialCommunityIcons from 'react-native-vector-icons/MaterialCommunityIcons';
import MaterialIcons from 'react-native-vector-icons/MaterialIcons';

async function loadFonts(): Promise<void> {
	await Promise.all([
		AntIcon.loadFont(),
		MaterialIcons.loadFont(),
		MaterialCommunityIcons.loadFont(),
		FeatherIcon.loadFont(),
		FontAwesome.loadFont()
	]);
}

loadFonts();
