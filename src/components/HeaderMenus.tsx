// Copyright 2015-2021 Parity Technologies (UK) Ltd.
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

import { useNavigation, useRoute } from '@react-navigation/native';
import React, { useCallback } from 'react';
import { StyleSheet, View } from 'react-native';

import PopupMenu from './PopupMenu';

const MAIN_MENUS = ['LegacyAccountList', 'Main'];

function HeaderMenus(): React.ReactElement {
	const { navigate } = useNavigation()
	const { name } = useRoute();

	const showMainMenu = MAIN_MENUS.includes(name);

	const onAccountCreate = useCallback((to: string) => {
		navigate(to);
	}, [navigate])

	return (
		<View style={styles.body}>
			{showMainMenu && (
				<PopupMenu
					menuItems={[
						{ text: 'Add account', value: 'AccountNew' },
						{ text: 'Manage networks', value: 'NetworkSettings' }
					]}
					menuTriggerIconName={'add'}
					onSelect={onAccountCreate}
				/>
			)}
		</View>
	);
}

const styles = StyleSheet.create({
	body: {
		display: 'flex'
	}
});

export default HeaderMenus;
