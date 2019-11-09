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

import { Metadata } from '@polkadot/types';
import { Container } from 'unstated';

import { getMetadataByKey, saveNewMetadata } from '../util/db';
import { base64ToHex } from '../util/strings';

type State = {
	new: typeof Metadata,
	selected: typeof Metadata
};

const DEFAULT_STATE = Object.freeze({
	new: null,
	selected: null
});

export default class MetadataStore extends Container<State> {
	state = DEFAULT_STATE;

	async saveNew(blob, networkKey) {
		try {
			await saveNewMetadata(blob, networkKey);
		} catch (e) {
			debugger;
			throw new Error(e);
		}
	}

	async getMetaByKey(networkKey) {
		try {
			const blob = await getMetadataByKey(networkKey);

			const result = new Metadata(base64ToHex(blob));

			return result;
		} catch (e) {
			throw new Error(e);
		}
	}

	// async updateMedata(networkKey) {}
}
