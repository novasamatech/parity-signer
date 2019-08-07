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

'use strict';

import React from 'react';
import { Text } from 'react-native';
import {
    Menu,
    MenuOptions,
    MenuOption,
    MenuTrigger,
} from 'react-native-popup-menu';
import Icon from 'react-native-vector-icons/MaterialIcons';
import colors from '../colors';
import fonts from "../fonts";

export default class PopupMenu extends React.PureComponent {
    render() {
        const { onSelect, menuTriggerIconName, menuItems } = this.props;
        const menuTriggerIcon = <Icon name={menuTriggerIconName} size={35} color={colors.bg_text_sec} />;

        return (
            <Menu onSelect={onSelect}>
                <MenuTrigger children={menuTriggerIcon} />
                <MenuOptions customStyles={menuOptionsStyles}>
                    {
                        menuItems.map((menuItem, index) => (
                            <MenuOption key={index} value={menuItem.value} >
                                <Text style={[menuOptionsStyles.optionText, menuItem.textStyle]} >{menuItem.text}</Text>
                            </MenuOption>
                        ))
                    }
                </MenuOptions>
            </Menu>
        );
    }
}

const menuOptionsStyles = {
    optionWrapper: {
        padding: 15,
    },
    optionText: {
        fontFamily: fonts.regular,
        fontSize: 16
    },
};
