import React from 'react';
import { Keyboard, Platform } from 'react-native';
import { KeyboardAwareScrollView } from 'react-native-keyboard-aware-scroll-view';

class KeyboardScrollView extends React.Component {
	render() {
		const defaultProps = { enableAutomaticScroll: true };
		return Platform.select({
			android: (
				<KeyboardAwareScrollView
					keyboardDismissMode="on-drag"
					onScrollEndDrag={Keyboard.dismiss}
					keyboardShouldPersistTaps="handled"
					enableOnAndroid
					{...defaultProps}
					{...this.props}
				/>
			),
			ios: (
				<KeyboardAwareScrollView
					keyboardDismissMode="interactive"
					keyboardShouldPersistTaps="handled"
					{...defaultProps}
					{...this.props}
				/>
			)
		});
	}
}

export default KeyboardScrollView;
