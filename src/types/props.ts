import {
	NavigationInjectedProps,
	NavigationScreenProp
} from 'react-navigation';
import {
	GestureResponderEvent,
	NativeSyntheticEvent,
	TextInputChangeEventData,
	TextInputFocusEventData
} from 'react-native';
import AccountsStore from '../stores/AccountsStore';

export interface NavigationProps<Params> {
	navigation: NavigationScreenProp<ScreenProps, Params>;
}

export type ScreenProps = NavigationInjectedProps;

export type ButtonListener = (event: GestureResponderEvent) => void;
export type TextChangeListener = (
	event: NativeSyntheticEvent<TextInputChangeEventData>
) => void;
export type FocusListener = (
	event: NativeSyntheticEvent<TextInputFocusEventData>
) => void;

export interface NavigationAccountProps<Params> {
	accounts: AccountsStore;
	navigation: NavigationScreenProp<{}, Params>;
}
