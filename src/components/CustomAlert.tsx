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

import Button from 'components/Button';
import React, { useContext, useEffect, useMemo, useState } from 'react';
import { Animated, Easing,StyleSheet, Text, View } from 'react-native';
import colors from 'styles/colors';
import fonts from 'styles/fonts';
import fontStyles from 'styles/fontStyles';

import { Action, AlertContext } from '../context';

export default function CustomAlert(): React.ReactElement {
	const { actions, alertIndex, message, title } = useContext(AlertContext);
	/* eslint-disable-next-line react-hooks/exhaustive-deps */
	const animatedValue = useMemo(() => new Animated.Value(1), [alertIndex]);
	const [alertDisplay, setAlertDisplay] = useState<boolean>(false);

	useEffect(() => {
		if (alertIndex === 0) return;
		setAlertDisplay(true);

		if (actions.length === 0) {
			Animated.timing(animatedValue, {
				duration: 1000,
				easing: Easing.poly(8),
				toValue: 0,
				useNativeDriver: false
			}).start(() => {
				setAlertDisplay(false);
			});
		}
		/* eslint-disable-next-line react-hooks/exhaustive-deps */
	}, [alertIndex]);

	const renderActions = (action: Action, index: number): React.ReactElement => (
		<Button
			key={'alert' + index}
			onPress={(): any => {
				if (action.onPress) {
					action.onPress();
				}

				setAlertDisplay(false);
			}}
			onlyText={true}
			small={true}
			style={styles.button}
			testID={action.testID}
			textStyles={
				action.onPress ? styles.buttonBoldText : styles.buttonLightText
			}
			title={action.text}
		/>
	);

	if (alertDisplay) {
		return (
			<Animated.View style={{ ...styles.background, opacity: animatedValue }}>
				<View style={styles.body}>
					{title !== '' && <Text style={styles.textTitle}>{title}</Text>}
					<Text style={styles.textMessage}>{message}</Text>
					{actions !== [] && (
						<View style={styles.actionsContainer}>
							{actions.map(renderActions)}
						</View>
					)}
				</View>
			</Animated.View>
		);
	} else {
		return <View />;
	}
}

const styles = StyleSheet.create({
	actionsContainer: {
		flexDirection: 'row',
		justifyContent: 'space-around',
		marginTop: 20
	},
	background: {
		alignItems: 'center',
		justifyContent: 'center',
		position: 'absolute',
		top: 80,
		width: '100%',
		zIndex: 100
	},
	body: {
		backgroundColor: colors.background.alert,
		paddingHorizontal: 20,
		paddingVertical: 20,
		width: '90%'
	},
	button: {
		alignItems: 'center',
		borderRadius: 0,
		flex: 1,
		flexGrow: 1,
		justifyContent: 'center',
		marginHorizontal: 2,
		paddingHorizontal: 0
	},

	buttonBoldText: { fontFamily: fonts.robotoMonoMedium },
	buttonLightText: { fontFamily: fonts.robotoMono },
	textMessage: { ...fontStyles.h2 },
	textTitle: {
		paddingVertical: 10,
		...fontStyles.h1
	}
});
