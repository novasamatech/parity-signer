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

import React, { useState } from 'react';

import { deepCopyMap } from 'stores/utils';
import { NetworkParams } from 'types/networkTypes';
import { loadAccountTxs, saveTx as saveTxDB } from 'utils/db';
import { TxParticipant } from 'types/tx';

type TxContextState = {
	saveTx: (tx: any, allNetworks: Map<string, NetworkParams>) => Promise<void>;
	getTxList: ({ address }: { address: string }) => string[];
	loadTxsForAccount: (
		account: TxParticipant,
		allNetworks: Map<string, NetworkParams>
	) => Promise<void>;
	signedTxs: Map<string, Record<string, any>>;
};

export function useTxStore(): TxContextState {
	const [signedTxs, setSignedTxs] = useState(new Map());

	async function saveTx(
		tx: any,
		allNetworks: Map<string, NetworkParams>
	): Promise<void> {
		await saveTxDB(tx, allNetworks);
		const newSignedTxs = deepCopyMap(signedTxs);
		signedTxs.set(tx.hash, tx);
		setSignedTxs(newSignedTxs);
	}

	async function loadTxsForAccount(
		account: TxParticipant,
		allNetworks: Map<string, NetworkParams>
	): Promise<void> {
		const txs = await loadAccountTxs(account, allNetworks);
		const newSignedTxs = new Map([...signedTxs, ...txs]);
		setSignedTxs(newSignedTxs);
	}

	function getTxList({ address }: { address: string }): string[] {
		return Array.from(signedTxs.values()).filter(
			tx => tx.sender === address || tx.recipient === address
		);
	}

	return {
		getTxList,
		loadTxsForAccount,
		saveTx,
		signedTxs
	};
}

export const TxContext = React.createContext({} as TxContextState);
