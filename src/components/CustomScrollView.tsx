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
import {
	Animated,
	LayoutChangeEvent,
	ScrollView,
	StyleSheet,
	ViewStyle
} from 'react-native';

import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import colors from 'styles/colors';

export default class CustomScrollView extends React.PureComponent<
	{
		contentContainerStyle: ViewStyle;
	},
	{
		indicator: Animated.AnimatedValue;
		visible: boolean;
		visibleHeight: number;
		wholeHeight: number;
	}
> {
	state = {
		indicator: new Animated.Value(0),
		visible: false,
		visibleHeight: 0,
		wholeHeight: 1
	};

	render(): React.ReactElement {
		const indicatorSize =
			this.state.wholeHeight > this.state.visibleHeight
				? (this.state.visibleHeight * this.state.visibleHeight) /
				  this.state.wholeHeight
				: this.state.visibleHeight;

		const difference =
			this.state.visibleHeight > indicatorSize
				? this.state.visibleHeight - indicatorSize
				: 1;

		return (
			<SafeAreaViewContainer>
				<ScrollView
					bounces={false}
					showsVerticalScrollIndicator={false}
					onContentSizeChange={(width: number, height: number): void => {
						this.setState({ wholeHeight: height });
					}}
					onLayout={({
						nativeEvent: {
							layout: { height }
						}
					}: LayoutChangeEvent): void =>
						this.setState({ visibleHeight: height })
					}
					scrollEventThrottle={16}
					onScroll={Animated.event([
						{ nativeEvent: { contentOffset: { y: this.state.indicator } } }
					])}
					{...this.props}
				>
					{this.props.children}
				</ScrollView>
				<Animated.View
					style={[
						styles.indicator,
						{
							height: indicatorSize,
							transform: [
								{
									translateY: Animated.multiply(
										this.state.indicator,
										this.state.visibleHeight / this.state.wholeHeight
									).interpolate({
										extrapolate: 'clamp',
										inputRange: [0, difference],
										outputRange: [0, difference]
									})
								}
							]
						}
					]}
				/>
			</SafeAreaViewContainer>
		);
	}
}

const styles = StyleSheet.create({
	indicator: {
		backgroundColor: colors.bg_text_sec,
		borderRadius: 5,
		position: 'absolute',
		right: 0,
		width: 5
	}
});
