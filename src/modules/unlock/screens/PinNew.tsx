// Copyright 2015-2021 Parity Technologies (UK) Ltd.
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

import ScreenHeading from 'components/ScreenHeading';
import testIDs from 'e2e/testIDs';
import { KeyboardAwareContainer } from 'modules/unlock/components/Container';
import PinInput from 'modules/unlock/components/PinInput';
import { usePinState } from 'modules/unlock/hooks';
import t from 'modules/unlock/strings';
import { getSubtitle, onPinInputChange } from 'modules/unlock/utils';
import { NavigationProps } from 'types/props';
import Button from 'components/Button';

export default function PinNew({
	route
}: NavigationProps<'PinNew'>): React.ReactElement {
	const [state, updateState, resetState] = usePinState();

	function submit(): void {
		const { pin, confirmation } = state;
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
			contentContainerStyle={{
				flexGrow: 1
			}}
		>
			<ScreenHeading
				title={t.title.pinCreation}
				subtitle={getSubtitle(state, false)}
				error={state.pinMismatch || state.pinTooShort}
			/>
			<PinInput
				label={t.pinLabel}
				autoFocus
				testID={testIDs.IdentityPin.setPin}
				returnKeyType="next"
				onFocus={(): void => updateState({ focusConfirmation: false })}
				onSubmitEditing={(): void => {
					updateState({ focusConfirmation: true });
				}}
				onChangeText={onPinInputChange('pin', updateState)}
				value={state.pin}
			/>
			<PinInput
				label={t.pinConfirmLabel}
				returnKeyType="done"
				testID={testIDs.IdentityPin.confirmPin}
				focus={state.focusConfirmation}
				onChangeText={onPinInputChange('confirmation', updateState)}
				value={state.confirmation}
				onSubmitEditing={submit}
			/>
			<Button
				title={t.doneButton.pinCreation}
				onPress={submit}
				testID={testIDs.IdentityPin.submitButton}
				aboveKeyboard
			/>
		</KeyboardAwareContainer>
	);
}
