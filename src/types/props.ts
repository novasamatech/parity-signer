import {RouteProp} from '@react-navigation/native';
import {
	GestureResponderEvent,
	NativeSyntheticEvent,
	TextInputChangeEventData,
	TextInputFocusEventData,
} from 'react-native';
import { StackNavigationProp } from '@react-navigation/stack';

import AccountsStore from 'stores/AccountsStore';
import ScannerStore from 'stores/ScannerStore';
import {RootStackParamList} from 'types/router';

export interface NavigationProps<ScreenName> {
	route: RouteProp<RootStackParamList, ScreenName>,
	navigation: StackNavigationProp<
		RootStackParamList,
		ScreenName
		>;
}

export type ButtonListener = (event: GestureResponderEvent) => void;
export type TextChangeListener = (
	event: NativeSyntheticEvent<TextInputChangeEventData>
) => void;
export type FocusListener = (
	event: NativeSyntheticEvent<TextInputFocusEventData>
) => void;

export interface NavigationAccountProps<ScreenName>
	extends NavigationProps<ScreenName> {
	accounts: AccountsStore;
}

export interface NavigationAccountScannerProps<ScreenName>
	extends NavigationAccountProps<ScreenName> {
	scannerStore: ScannerStore;
}

export interface NavigationScannerProps<ScreenName>
	extends NavigationProps<ScreenName> {
	scannerStore: ScannerStore;
}
