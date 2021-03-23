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

import PropTypes from 'prop-types';
import React from 'react';

import PathCard from './PathCard';

import { AccountsContextState } from 'stores/AccountsContext';
import { FoundAccount } from 'types/identityTypes';

const CompatibleCard = ({
	account,
	accountsStore,
	titlePrefix
}: {
	account: FoundAccount;
	accountsStore: AccountsContextState;
	titlePrefix?: string;
}): React.ReactElement => {
	const renderIdentityPathCard = (
		identityAccount: FoundAccount
	): React.ReactElement => {
		const identity = accountsStore.getIdentityByAccountId(
			identityAccount.accountId
		)!;
		return (
			<PathCard
				identity={identity}
				path={identityAccount.path}
				titlePrefix={titlePrefix + identity.name}
			/>
		);
	};

	return renderIdentityPathCard(account);
};

CompatibleCard.propTypes = {
	account: PropTypes.object.isRequired,
	accountsStore: PropTypes.object.isRequired,
	titlePrefix: PropTypes.string
};

export default CompatibleCard;
