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

import Button from 'components/Button';
import ScreenHeading from 'components/ScreenHeading';
import testIDs from 'e2e/testIDs';
import { KeyboardAwareContainer } from 'modules/unlock/components/Container';
import PinInput from 'modules/unlock/components/PinInput';
import { usePinState } from 'modules/unlock/hooks';
import t from 'modules/unlock/strings';
import { getSubtitle, onPinInputChange } from 'modules/unlock/utils';
import React, { useState } from 'react';
import { NavigationTargetIdentityProps } from 'types/props';
import { withTargetIdentity } from 'utils/HOC';
import { useSeedRef } from 'utils/seedRefHooks';

function PinUnlockWithPassword({ route, targetIdentity }: NavigationTargetIdentityProps<'PinUnlockWithPassword'>): React.ReactElement {
	const [state, updateState, resetState] = usePinState();
	const [focusPassword, setFocusPassword] = useState<boolean>(false);
	const { createSeedRef } = useSeedRef(targetIdentity.encryptedSeed);

	async function submit(): Promise<void> {
		const { password, pin } = state;
		const resolvePassword = route.params.resolve;

		if (!route.params.isSeedRefValid) {
			if (pin.length >= 6 && targetIdentity) {
				try {
					await createSeedRef(pin);
					resolvePassword(password);
					resetState();
				} catch (e) {
					updateState({ password: '', pin: '', pinMismatch: true });
					//TODO record error times;
				}
			} else {
				updateState({ pinTooShort: true });
			}
		} else {
			resolvePassword(password);
			resetState();
		}
	}

	function onPasswordInputChange(password: string): void {
		updateState({
			password,
			pinMismatch: false
		});
	}

	return (
		<KeyboardAwareContainer
			contentContainerStyle={{ flexGrow: 1 }}
		>
			<ScreenHeading
				error={state.pinMismatch || state.pinTooShort}
				subtitle={getSubtitle(state, true)}
				title={t.title.pinUnlock}
			/>
			{!route.params.isSeedRefValid && (
				<PinInput
					autoFocus
					label={t.pinLabel}
					onChangeText={onPinInputChange('pin', updateState)}
					onSubmitEditing={(): void => setFocusPassword(true)}
					returnKeyType="done"
					testID={testIDs.IdentityPin.unlockPinInput}
					value={state.pin}
				/>
			)}
			<PinInput
				focus={focusPassword}
				keyboardType="default"
				label={t.passwordLabel}
				onChangeText={onPasswordInputChange}
				onSubmitEditing={submit}
				returnKeyType="done"
				testID={testIDs.IdentityPin.passwordInput}
				value={state.password}
			/>
			<Button
				onPress={submit}
				testID={testIDs.IdentityPin.unlockPinButton}
				title={t.doneButton.pinUnlock}
			/>
		</KeyboardAwareContainer>
	);
}

export default withTargetIdentity(PinUnlockWithPassword);
