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

import { useState } from 'react';

import { State, StateReducer, UpdateStateFunc } from './types';

const initialState: State = {
	confirmation: '',
	focusConfirmation: false,
	password: '',
	pin: '',
	pinMismatch: false,
	pinTooShort: false
};

export function usePinState(): StateReducer {
	const [state, setState] = useState<State>(initialState);
	const updateState: UpdateStateFunc = delta =>
		setState({ ...state, ...delta });
	const resetState = (): void => setState(initialState);
	return [state, updateState, resetState];
}
