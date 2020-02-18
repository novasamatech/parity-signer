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

import { rlpItem } from './native';
import { fromWei } from './units';

export class Transaction {
	readonly nonce: string;
	readonly gasPrice: string;
	readonly gas: string;
	readonly action: string;
	readonly value: string;
	readonly data: string;
	readonly ethereumChainId: string;
	readonly isSafe: boolean;

	constructor(
		nonce: string,
		gasPrice: string,
		gas: string,
		action: string,
		value: string,
		data: string,
		ethereumChainId: string
	) {
		this.nonce = nonce || '0';
		this.gasPrice = parseInt(gasPrice, 16).toString();
		this.gas = parseInt(gas, 16).toString();
		this.action = action;
		this.value = fromWei(value);
		this.data = data || '-';
		this.ethereumChainId = parseInt(ethereumChainId, 16).toString();
		this.isSafe = true;
	}
}

async function asyncTransaction(
	rlp: string,
	resolve: (value?: Transaction) => void,
	reject: any
): Promise<void> {
	try {
		const nonce = await rlpItem(rlp, 0);
		const gasPrice = await rlpItem(rlp, 1);
		const gas = await rlpItem(rlp, 2);
		const action = await rlpItem(rlp, 3);
		const value = await rlpItem(rlp, 4);
		const data = await rlpItem(rlp, 5);
		const ethereumChainId = await rlpItem(rlp, 6);
		const tx = new Transaction(
			nonce,
			gasPrice,
			gas,
			action,
			value,
			data,
			ethereumChainId
		);
		resolve(tx);
	} catch (e) {
		reject(e);
	}
}

export default function transaction(rlp: string): Promise<Transaction> {
	return new Promise((resolve, reject) =>
		asyncTransaction(rlp, resolve, reject)
	);
}
