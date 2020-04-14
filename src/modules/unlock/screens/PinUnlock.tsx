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

import React from 'react';

import Container from 'modules/unlock/components/Container';
import PinInput from 'modules/unlock/components/PinInput';
import { usePinState } from 'modules/unlock/hooks';
import t from 'modules/unlock/strings';
import { getSubtitle, onPinInputChange } from 'modules/unlock/utils';
import testIDs from 'e2e/testIDs';
import ScreenHeading from 'components/ScreenHeading';
import ButtonMainAction from 'components/ButtonMainAction';
import { NavigationAccountProps } from 'types/props';
import { withAccountStore } from 'utils/HOC';
import {
	unlockIdentitySeed,
	unlockIdentitySeedWithReturn
} from 'utils/identitiesUtils';
import { useSeedRef } from 'utils/seedRefHooks';

function PinUnlock({
	accounts,
	route
}: NavigationAccountProps<'PinUnlock'>): React.ReactElement {
	const [state, updateState, resetState] = usePinState();
	const { createSeedRef } = useSeedRef();
	const targetIdentity =
		route.params.identity ?? accounts.state.currentIdentity;

	async function submit(): Promise<void> {
		const { pin } = state;
		if (pin.length >= 6 && targetIdentity) {
			try {
				const resolve = route.params.resolve;
				if (route.params.shouldReturnSeed) {
					const seedPhrase = await unlockIdentitySeedWithReturn(
						pin,
						targetIdentity,
						createSeedRef
					);
					resetState();
					resolve(seedPhrase);
				} else {
					await unlockIdentitySeed(pin, targetIdentity, createSeedRef);
					resetState();
					resolve();
				}
			} catch (e) {
				updateState({ pin: '', pinMismatch: true });
				//TODO record error times;
			}
		} else {
			updateState({ pinTooShort: true });
		}
	}
	return (
		<Container>
			<ScreenHeading
				title={t.title.pinUnlock}
				error={state.pinMismatch || state.pinTooShort}
				subtitle={getSubtitle(state, true)}
			/>
			<PinInput
				label={t.pinLabel}
				autoFocus
				testID={testIDs.IdentityPin.unlockPinInput}
				returnKeyType="done"
				onChangeText={onPinInputChange('pin', updateState)}
				onSubmitEditing={submit}
				value={state.pin}
			/>
			<ButtonMainAction
				title={t.doneButton.pinUnlock}
				bottom={false}
				onPress={submit}
				testID={testIDs.IdentityPin.unlockPinButton}
			/>
		</Container>
	);
}

export default withAccountStore(PinUnlock);
