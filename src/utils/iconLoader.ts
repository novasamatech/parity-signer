import AntIcon from 'react-native-vector-icons/AntDesign';
import MaterialIcons from 'react-native-vector-icons/MaterialIcons';
import MaterialCommunityIcons from 'react-native-vector-icons/MaterialCommunityIcons';
import FeatherIcon from 'react-native-vector-icons/Feather';
import FontAwesome from 'react-native-vector-icons/FontAwesome';
import FontAwesome5 from 'react-native-vector-icons/FontAwesome5';

async function loadFonts () {
	await Promise.all([
		AntIcon.loadFont(),
		MaterialIcons.loadFont(),
		MaterialCommunityIcons.loadFont(),
		FeatherIcon.loadFont(),
		FontAwesome.loadFont(),
		FontAwesome5.loadFont(),
	]);
}
loadFonts();
