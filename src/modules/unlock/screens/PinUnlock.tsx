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

import { KeyboardAwareContainer } from 'modules/unlock/components/Container';
import PinInput from 'modules/unlock/components/PinInput';
import { usePinState } from 'modules/unlock/hooks';
import t from 'modules/unlock/strings';
import { getSubtitle, onPinInputChange } from 'modules/unlock/utils';
import testIDs from 'e2e/testIDs';
import ScreenHeading from 'components/ScreenHeading';
import { NavigationTargetIdentityProps } from 'types/props';
import { debounce } from 'utils/debounce';
import { withTargetIdentity } from 'utils/HOC';
import { unlockIdentitySeedWithReturn } from 'utils/identitiesUtils';
import { decryptData } from 'utils/native';
import { useSeedRef } from 'utils/seedRefHooks';

function PinUnlock({
	targetIdentity,
	route
}: NavigationTargetIdentityProps<'PinUnlock'>): React.ReactElement {
	const [state, updateState, resetState] = usePinState();
	const { createSeedRef } = useSeedRef(targetIdentity.encryptedSeed);

	async function submit(pin: string): Promise<void> {
		if (pin.length >= 6 && targetIdentity) {
			try {
				if (route.params.shouldReturnSeed) {
					const resolveSeedPhrase = route.params.resolve;
					const seedPhrase = await unlockIdentitySeedWithReturn(
						pin,
						targetIdentity,
						createSeedRef
					);
					resetState();
					resolveSeedPhrase(seedPhrase);
				} else {
					const resolve = route.params.resolve;
					await decryptData(targetIdentity.encryptedSeed, pin);
					await createSeedRef(pin);
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
			contentContainerStyle={{
				flexGrow: 1
			}}
		>
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
				onChangeText={onPinInput}
				value={state.pin}
			/>
		</KeyboardAwareContainer>
	);
}

export default withTargetIdentity(PinUnlock);
