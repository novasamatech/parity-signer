// Copyright 2015-2019 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

import React from 'react';
import { Keyboard, Platform, ScrollViewProps } from 'react-native';
import { KeyboardAwareScrollView } from 'react-native-keyboard-aware-scroll-view';

import { SafeAreaViewContainer } from 'components/SafeAreaContainer';

interface Props extends ScrollViewProps {
	enableAutomaticScroll?: boolean;
	extraHeight?: number;
}

class KeyboardScrollView extends React.PureComponent<Props> {
	render(): React.ReactElement | undefined {
		const defaultProps = { enableAutomaticScroll: true };
		return Platform.select({
			android: (
				<SafeAreaViewContainer>
					<KeyboardAwareScrollView
						keyboardDismissMode="on-drag"
						onScrollEndDrag={Keyboard.dismiss}
						keyboardShouldPersistTaps="handled"
						enableOnAndroid
						{...defaultProps}
						{...this.props}
					>
						{this.props.children}
					</KeyboardAwareScrollView>
				</SafeAreaViewContainer>
			),
			ios: (
				<SafeAreaViewContainer>
					<KeyboardAwareScrollView
						keyboardDismissMode="interactive"
						keyboardShouldPersistTaps="handled"
						{...defaultProps}
						{...this.props}
					>
						{this.props.children}
					</KeyboardAwareScrollView>
				</SafeAreaViewContainer>
			)
		});
	}
}

export default KeyboardScrollView;
