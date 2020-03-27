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

import React, { useRef } from 'react';

import ButtonMainAction from 'components/ButtonMainAction';
import ScreenHeading from 'components/ScreenHeading';
import TextInput from 'components/TextInput';
import testIDs from 'e2e/testIDs';
import Container from 'modules/unlock/components/Container';
import PinInput from 'modules/unlock/components/PinInput';
import { usePinState } from 'modules/unlock/hooks';
import t from 'modules/unlock/strings';
import { getSubtitle, onPinInputChange } from 'modules/unlock/utils';
import { NavigationAccountProps } from 'types/props';
import { unlockIdentitySeed, verifyPassword } from 'utils/identitiesUtils';
import { constructSURI } from 'utils/suri';

export default function PinUnlockWithPassword({
	accounts,
	route
}: NavigationAccountProps<'PinUnlockWithPassword'>): React.ReactElement {
	const [state, updateState, resetState] = usePinState();
	const passwordInput = useRef<TextInput>(null);
	const targetIdentity =
		route.params.identity ?? accounts.state.currentIdentity;

	async function submit(): Promise<void> {
		const { pin, password } = state;
		const derivePath = route.params.path;
		if (pin.length >= 6 && targetIdentity) {
			try {
				const resolve = route.params.resolve;
				const seedPhrase = await unlockIdentitySeed(pin, targetIdentity);
				const isPasswordValid = verifyPassword(
					password,
					seedPhrase,
					targetIdentity,
					derivePath
				);
				if (isPasswordValid) {
					const suri = constructSURI({
						derivePath,
						password,
						phrase: seedPhrase
					});
					resetState();
					resolve(suri);
				} else {
					updateState({ pin: '', pinMismatch: true });
				}
			} catch (e) {
				updateState({ pin: '', pinMismatch: true });
				//TODO record error times;
			}
		} else {
			updateState({ pinTooShort: true });
		}
	}

	function onPasswordInputChange(password: string) {
		updateState({
			pinMismatch: false,
			password
		});
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
				onSubmitEditing={(): void => passwordInput.current?.input?.focus()}
				value={state.pin}
			/>
			<PinInput
				label={t.passwordLabel}
				autoFocus
				testID={testIDs.IdentityPin.passwordInput}
				returnKeyType="done"
				ref={passwordInput}
				onChangeText={onPasswordInputChange}
				onSubmitEditing={submit}
				value={state.password}
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
