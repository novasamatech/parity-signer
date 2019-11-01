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
import { Subscribe } from 'unstated';
import AccountsStore from '../stores/AccountsStore';
import { Text } from 'react-native';

export default class PathsList extends React.PureComponent {
	static navigationOptions = {
		headerBackTitle: 'Back',
		title: 'Paths List'
	};
	render() {
		return (
			<Subscribe to={[AccountsStore]}>
				{accounts => <PathsListView {...this.props} accounts={accounts} />}
			</Subscribe>
		);
	}
}

function PathsListView({ accounts }) {
	const paths = Array.from(accounts.state.currentIdentity.meta.keys());
	return (
		<>
			{paths.map(path => (
				<Text>{path}</Text>
			))}
		</>
	);
}
