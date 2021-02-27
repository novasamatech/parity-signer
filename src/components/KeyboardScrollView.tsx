// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// Modifications Copyright (c) 2021 Thibaut Sardan

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import React from 'react';
import { Keyboard, Platform, ScrollViewProps } from 'react-native';
import { KeyboardAwareScrollView } from 'react-native-keyboard-aware-scroll-view';

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
						enableOnAndroid
						keyboardDismissMode="on-drag"
						keyboardShouldPersistTaps="handled"
						onScrollEndDrag={Keyboard.dismiss}
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
