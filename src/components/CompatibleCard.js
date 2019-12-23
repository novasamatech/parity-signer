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

import PropTypes from 'prop-types';
import AccountCard from './AccountCard';
import PathCard from './PathCard';
import React from 'react';
import { getIdentityName } from '../util/identitiesUtils';
import { defaultNetworkKey } from '../constants';

const CompatibleCard = ({ account, accountsStore, titlePrefix }) =>
	account.isLegacy === true || account.isLegacy === undefined ? (
		<AccountCard
			title={account.name}
			address={account.address}
			networkKey={account.networkKey || ''}
		/>
	) : account.path === '' ? (
		<AccountCard
			title={getIdentityName(account, accountsStore.state.identities)}
			address={account.address}
			networkKey={defaultNetworkKey}
		/>
	) : (
		<PathCard
			identity={accountsStore.getIdentityByAccountId(account.accountId)}
			path={account.path}
			titlePrefix={titlePrefix}
		/>
	);

CompatibleCard.propTypes = {
	account: PropTypes.object.isRequired,
	accountsStore: PropTypes.object.isRequired,
	titlePrefix: PropTypes.string
};

export default CompatibleCard;
