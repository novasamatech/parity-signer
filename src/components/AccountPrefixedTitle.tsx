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

import React, { ReactElement } from 'react';
import { Text, View } from 'react-native';

import fontStyles from 'styles/fontStyles';
import colors from 'styles/colors';

export default function AccountPrefixedTitle({
	titlePrefix,
	title
}: {
	title: string;
	titlePrefix?: string;
}): ReactElement {
	return (
		<View style={{ flexDirection: 'row' }}>
			{titlePrefix && (
				<Text
					numberOfLines={1}
					style={[
						fontStyles.t_codeS,
						{ color: colors.bg_button, marginTop: 5 }
					]}
				>
					{titlePrefix}
				</Text>
			)}
			<Text numberOfLines={1} style={[fontStyles.h2, { marginTop: -2 }]}>
				{title}
			</Text>
		</View>
	);
}
