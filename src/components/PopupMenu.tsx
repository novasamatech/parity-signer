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

import React from 'react';
import { Text, TextStyle } from 'react-native';
import { Menu, MenuOption, MenuOptions, MenuTrigger } from 'react-native-popup-menu';
import Icon from 'react-native-vector-icons/MaterialIcons';
import colors from 'styles/colors';
import fonts from 'styles/fonts';

type MenuItem = {
	text: string;
	value: string;
	textStyle?: TextStyle;
	testID?: string;
	hide?: boolean;
};

interface Props {
	onSelect: (selectedItem: any) => void;
	menuTriggerIconName: string;
	menuItems: Array<MenuItem>;
	testID?: string;
}

export default class PopupMenu extends React.PureComponent<Props> {
	render(): React.ReactElement {
		const { menuItems, menuTriggerIconName, onSelect, testID } = this.props;
		const menuTriggerIcon = (
			<Icon
				color={colors.text.main}
				name={menuTriggerIconName}
				size={35}
				testID={testID}
			/>
		);

		return (
			<Menu onSelect={onSelect}>
				<MenuTrigger children={menuTriggerIcon} />
				<MenuOptions customStyles={menuOptionsStyles}>
					{menuItems.map((menuItem: MenuItem, index: number): React.ReactNode => {
						if (menuItem.hide === true) {
							return null;
						}

						return (
							<MenuOption key={index}
								value={menuItem.value}>
								<Text
									style={[menuOptionsStyles.optionText, menuItem.textStyle]}
									testID={menuItem.testID}
								>
									{menuItem.text}
								</Text>
							</MenuOption>
						);
					})}
				</MenuOptions>
			</Menu>
		);
	}
}

const menuOptionsStyles = {
	optionText: {
		fontFamily: fonts.regular,
		fontSize: 16
	},
	optionWrapper: {
		padding: 15
	}
};
