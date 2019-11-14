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

// @flow

import PropTypes from 'prop-types';
import React from 'react';
import { View, Text } from 'react-native';
import AntIcon from 'react-native-vector-icons/AntDesign';
import fontStyles from '../fontStyles';
import fonts from '../fonts';
import ButtonIcon from './ButtonIcon';
import { Icon } from 'react-native-elements';
import colors from '../colors';

export default class ScreenHeading extends React.PureComponent {
	static propTypes = {
		big: PropTypes.bool,
		onPress: PropTypes.func,
		small: PropTypes.bool,
		subtitle: PropTypes.string,
		title: PropTypes.string
	};
	render() {
		const {
			big,
			title,
			small,
			subtitle,
			subtitleL,
			subtitleIcon,
			error,
			onPress,
			iconName,
			iconType
		} = this.props;
		const finalViewStyles = [styles.body];
		const finalTextStyles = [fontStyles.h1, styles.t_center];
		const finalSubtitleStyle = [fontStyles.t_codeS];
		const finalSubtitleIconStyle = [styles.subtitleIcon];

		if (big) {
			finalViewStyles.push(styles.bodyL);
			finalTextStyles.push(styles.t_left);
			finalSubtitleIconStyle.push({ justifyContent: 'flex-start' });
		} else if (small) {
			finalViewStyles.push(styles.bodyL);
			finalTextStyles.push([fontStyles.h2, styles.t_left, styles.t_normal]);
		}

		if (error) {
			finalSubtitleStyle.push(styles.t_error);
		}
		if (subtitleL) {
			finalSubtitleStyle.push({ textAlign: 'left' });
		}

		const renderSubtitle = () => {
			if (!subtitle) return;
			return (
				<View style={finalSubtitleIconStyle}>
					{renderSubtitleIcon()}
					<Text style={[finalTextStyles, finalSubtitleStyle]}>{subtitle}</Text>
				</View>
			);
		};
		const renderSubtitleIcon = () => {
			if (!subtitleIcon) return;
			return <AntIcon name="user" size={10} color={colors.bg_text_sec} />;
		};

		const renderBack = () => {
			if (!onPress) return;
			return (
				<ButtonIcon
					iconName="arrowleft"
					iconType="antdesign"
					onPress={onPress}
					style={styles.icon}
					iconBgStyle={{ backgroundColor: 'transparent' }}
				/>
			);
		};
		const renderIcon = () => {
			if (!iconName) return;
			return (
				<View style={[styles.icon, { paddingLeft: 16 }]}>
					<Icon name={iconName} type={iconType} color={colors.bg_text} />
				</View>
			);
		};

		return (
			<View style={finalViewStyles}>
				<Text style={finalTextStyles}>{title}</Text>
				{renderSubtitle()}
				{renderBack()}
				{renderIcon()}
			</View>
		);
	}
}

const styles = {
	body: {
		marginBottom: 16,
		paddingHorizontal: 16
	},
	bodyL: {
		paddingLeft: 72,
		paddingRight: 16
	},
	icon: {
		marginLeft: 5,
		marginTop: 0,
		position: 'absolute'
	},
	subtitleIcon: {
		alignItems: 'center',
		flexDirection: 'row',
		justifyContent: 'center'
	},
	t_center: {
		textAlign: 'center'
	},
	t_error: {
		color: colors.bg_alert
	},
	t_left: {
		textAlign: 'left'
	},
	t_normal: {
		fontFamily: fonts.roboto
	}
};
