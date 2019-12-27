import React from 'react';
import { Animated, ScrollView, StyleSheet, View } from 'react-native';

import colors from '../colors';

export default class CustomScrollview extends React.PureComponent {
	state = {
		indicator: new Animated.Value(0),
		visible: false,
		visibleHeight: 0,
		wholeHeight: 1
	};

	render() {
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
			<View style={this.props.containerStyle}>
				<ScrollView
					showsVerticalScrollIndicator={false}
					onContentSizeChange={(width, height) => {
						this.setState({ wholeHeight: height });
					}}
					onLayout={({
						nativeEvent: {
							layout: { height }
						}
					}) => this.setState({ visibleHeight: height })}
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
			</View>
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
