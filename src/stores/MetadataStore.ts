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

// @flow

import {Metadata, TypeRegistry} from '@polkadot/types';
import { Container } from 'unstated';

import { getMetadataByKey } from 'utils/db';
import { base64ToHex } from 'utils/strings';
const registry = new TypeRegistry();

//TODO move all metadata related function here with central registry;

type State = {
	new: Metadata | null;
	selected: Metadata | null;
};

const DEFAULT_STATE = Object.freeze({
	new: null,
	selected: null
});

export const getMetadata = (metadata: string): Metadata => new Metadata(registry, metadata);

export default class MetadataStore extends Container<State> {
	state: State = DEFAULT_STATE;

	// async saveNew(blob, networkKey) {
	// 	try {
	// 		await saveNewMetadata(blob, networkKey);
	// 	} catch (e) {
	// 		debugger;
	// 		throw new Error(e);
	// 	}
	// }

	async getMetaByKey(networkKey: string): Promise<Metadata> {
		try {
			const blob = await getMetadataByKey(networkKey);
			const result = new Metadata(registry, base64ToHex(blob));
			return result;
		} catch (e) {
			throw new Error(e);
		}
	}

	// async updateMedata(networkKey) {}
}
