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
import ScannerStore from '../stores/ScannerStore';

export interface NavigationProps<Params> {
	navigation: NavigationScreenProp<{}, Params>;
}

export type ScreenProps = NavigationInjectedProps;

export type ButtonListener = (event: GestureResponderEvent) => void;
export type TextChangeListener = (
	event: NativeSyntheticEvent<TextInputChangeEventData>
) => void;
export type FocusListener = (
	event: NativeSyntheticEvent<TextInputFocusEventData>
) => void;

export interface NavigationAccountProps<Params>
	extends NavigationProps<Params> {
	accounts: AccountsStore;
}

export interface NavigationAccountScannerProps<Params>
	extends NavigationAccountProps<Params> {
	scannerStore: ScannerStore;
}

export interface NavigationScannerProps<Params>
	extends NavigationProps<Params> {
	scannerStore: ScannerStore;
}
