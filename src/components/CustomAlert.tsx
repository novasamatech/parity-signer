// Copyright 2015-2020 Parity Technologies (UK) Ltd.
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

import Button from 'components/Button';
import React, {useContext, useEffect, useMemo, useState} from 'react';
import {StyleSheet, View, Animated, Text} from 'react-native';
import {AlertStateContext} from 'stores/alertContext';

import colors from 'styles/colors';
import fontStyles from 'styles/fontStyles';

export default function CustomAlert() {
	const {title, index, message} = useContext(AlertStateContext);
	const animatedValue = useMemo(()=>new Animated.Value(1), [index]);
	const [alertDisplay, setAlertDisplay] = useState<boolean>(false);

	useEffect(() => {
		setAlertDisplay(true);
		Animated.timing(animatedValue, {
			toValue: 0,
			duration: 2000,
			useNativeDriver: false
		}).start(()=>{
			setAlertDisplay(false);
		});
	}, [index]);
	 if(alertDisplay) {
		 return <Animated.View style={{...styles.background, opacity: animatedValue}}>
		 	<View style={styles.body}>
				{title !== '' && <Text style={styles.textTitle}>{title}</Text>}
				<Text style={styles.textMessage}>{message}</Text>
				<Button title={"OK"} onPress={()=>{}}/>
			</View>
		 </Animated.View>;
	 } else {
	 	return null;
	 }
}

const styles = StyleSheet.create({
	background: {
		position: 'absolute',
		zIndex: 100,
		top: 80,
		width: '100%',
		justifyContent: 'center',
		alignItems: 'center',
		padding: 20
	},
	body: {
		width: '90%',
		paddingHorizontal: 10,
		backgroundColor: colors.background.alert
	},
	textMessage: {
		paddingTop: 10,
		...fontStyles.h2
	},
	textTitle: {
		paddingTop: 10,
		...fontStyles.h1,
	}
});
