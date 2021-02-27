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

import { cleanup, device,init } from 'detox';
import adapter from 'detox/runners/jest/adapter';
import specReporter from 'detox/runners/jest/specReporter';
import testIDs from 'e2e/testIDs';
import { testTap, testVisible } from 'e2e/utils';

import { detox as config } from '../../package.json';

// Set the default timeout
jest.setTimeout(120000);
jasmine.getEnv().addReporter(adapter);

// This takes care of generating status logs on a per-spec basis. By default, jest only reports at file-level.
// This is strictly optional.
jasmine.getEnv().addReporter(specReporter);

const { TacScreen } = testIDs;

beforeAll(async () => {
	await init(config, { launchApp: false });

	if (device.getPlatform() === 'ios') {
		await device.clearKeychain();
	}

	await device.launchApp({ permissions: { camera: 'YES' } });
	await testVisible(TacScreen.tacView);
	await testTap(TacScreen.agreePrivacyButton);
	await testTap(TacScreen.agreeTacButton);
	await testTap(TacScreen.nextButton);
});

beforeEach(async () => {
	await adapter.beforeEach();
});

afterAll(async () => {
	await adapter.afterAll();
	await cleanup();
});
