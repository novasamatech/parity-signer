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

import ScreenHeading from 'components/ScreenHeading';
import testIDs from 'e2e/testIDs';
import { KeyboardAwareContainer } from 'modules/unlock/components/Container';
import PinInput from 'modules/unlock/components/PinInput';
import { usePinState } from 'modules/unlock/hooks';
import t from 'modules/unlock/strings';
import { getSubtitle, onPinInputChange } from 'modules/unlock/utils';
import React from 'react';
import { NavigationTargetIdentityProps } from 'types/props';
import { debounce } from 'utils/debounce';
import { withTargetIdentity } from 'utils/HOC';
import { unlockIdentitySeedWithReturn } from 'utils/identitiesUtils';
// import { useSeedRef } from 'utils/seedRefHooks';

function PinUnlock({ route, targetIdentity }: NavigationTargetIdentityProps<'PinUnlock'>): React.ReactElement {
	const [state, updateState, resetState] = usePinState();
	// const { createSeedRef } = useSeedRef(targetIdentity.encryptedSeed);

	async function submit(pin: string): Promise<void> {
		if (pin.length >= 6 && targetIdentity) {
			try {
				if (route.params.shouldReturnSeed) {
					const resolveSeedPhrase = route.params.resolve;
					const seedPhrase = await unlockIdentitySeedWithReturn(pin, targetIdentity);

					resetState();
					resolveSeedPhrase(seedPhrase);
				} else {
					const resolve = route.params.resolve;

					// await createSeedRef(pin);
					resetState();
					resolve();
				}
			} catch (e) {
				updateState({ pin, pinMismatch: true });
			}
		} else {
			updateState({ pin, pinTooShort: true });
		}
	}

	const onPinInput = (pin: string): void => {
		onPinInputChange('pin', updateState)(pin);
		const debounceSubmit = debounce(() => submit(pin), 500);

		debounceSubmit();
	};

	return (
		<KeyboardAwareContainer
			contentContainerStyle={{ flexGrow: 1 }}
		>
			<ScreenHeading
				error={state.pinMismatch || state.pinTooShort}
				subtitle={getSubtitle(state, true)}
				title={t.title.pinUnlock}
			/>
			<PinInput
				autoFocus
				label={t.pinLabel}
				onChangeText={onPinInput}
				returnKeyType="done"
				testID={testIDs.IdentityPin.unlockPinInput}
				value={state.pin}
			/>
		</KeyboardAwareContainer>
	);
}

export default withTargetIdentity(PinUnlock);
