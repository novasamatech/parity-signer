import React, { ReactElement } from 'react';
import { View, Text, StyleSheet } from 'react-native';

import { PayloadCardContent } from 'types/payload';
import colors from 'styles/colors';
import fontStyles from 'styles/fontStyles';

type CardProps = {
	indent: number;
	payload?: PayloadCardContent;
};

export function DefaultCard({
	indent,
	payload
}: CardProps): ReactElement {
	return (
		<View>
			<Text style={fontStyles.t_regular}>{JSON.stringify(payload)}</Text>
		</View>
	);
}

export function CallCard({
	indent,
	payload
}: CardProps): ReactElement {
	return (
		<View>
			<Text
				style={[
					fontStyles.t_important,
					{
						paddingLeft: indent + '0%'
					}
				]}
			>
				{payload.method}
				<Text style={fontStyles.t_regular}> from </Text>
				{payload.pallet}
			</Text>

		</View>
	);
}


