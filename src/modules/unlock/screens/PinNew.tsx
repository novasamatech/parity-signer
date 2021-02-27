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
import React from 'react';
import { NavigationProps } from 'types/props';

export default function PinNew({ route }: NavigationProps<'PinNew'>): React.ReactElement {
	const [state, updateState, resetState] = usePinState();

	function submit(): void {
		const { confirmation, pin } = state;

		if (pin.length >= 6 && pin === confirmation) {
			const resolve = route.params.resolve;

			resetState();
			resolve(pin);
		} else {
			if (pin.length < 6) {
				updateState({ pinTooShort: true });
			} else if (pin !== confirmation) updateState({ pinMismatch: true });
		}
	}

	return (
		<KeyboardAwareContainer
			contentContainerStyle={{ flexGrow: 1 }}
		>
			<ScreenHeading
				error={state.pinMismatch || state.pinTooShort}
				subtitle={getSubtitle(state, false)}
				title={t.title.pinCreation}
			/>
			<PinInput
				autoFocus
				label={t.pinLabel}
				onChangeText={onPinInputChange('pin', updateState)}
				onFocus={(): void => updateState({ focusConfirmation: false })}
				onSubmitEditing={(): void => {
					updateState({ focusConfirmation: true });
				}}
				returnKeyType="next"
				testID={testIDs.IdentityPin.setPin}
				value={state.pin}
			/>
			<PinInput
				focus={state.focusConfirmation}
				label={t.pinConfirmLabel}
				onChangeText={onPinInputChange('confirmation', updateState)}
				onSubmitEditing={submit}
				returnKeyType="done"
				testID={testIDs.IdentityPin.confirmPin}
				value={state.confirmation}
			/>
			<Button
				aboveKeyboard
				onPress={submit}
				testID={testIDs.IdentityPin.submitButton}
				title={t.doneButton.pinCreation}
			/>
		</KeyboardAwareContainer>
	);
}
