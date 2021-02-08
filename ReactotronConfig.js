import Reactotron from 'reactotron-react-native';

if (__DEV__) {
	Reactotron.configure({ host: '192.168.1.63' }) // controls connection & communication settings
		.useReactNative() // add all built-in react native plugins
		.connect(); // let's connect!
}
