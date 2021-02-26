import { RouteProp } from '@react-navigation/native';
import { StackNavigationProp } from '@react-navigation/stack';
import { GestureResponderEvent, NativeSyntheticEvent, TextInputChangeEventData, TextInputFocusEventData } from 'react-native';
import { AccountsStoreStateWithIdentity, LegacyAccount } from 'types/identityTypes';
import { RootStackParamList } from 'types/routes';

import { AccountsContextType, ScannerContextType } from '../context';

export interface NavigationProps<ScreenName extends keyof RootStackParamList> {
	route: RouteProp<RootStackParamList, ScreenName>;
	navigation: StackNavigationProp<RootStackParamList, ScreenName>;
}

export type ButtonListener = (event: GestureResponderEvent) => void;
export type TextChangeListener = (
	event: NativeSyntheticEvent<TextInputChangeEventData>
) => void;
export type FocusListener = (
	event: NativeSyntheticEvent<TextInputFocusEventData>
) => void;

export interface NavigationAccountIdentityProps<
	ScreenName extends keyof RootStackParamList
> extends NavigationProps<ScreenName> {
	accountsStore: AccountsStoreStateWithIdentity;
}

export interface NavigationTargetIdentityProps<
	ScreenName extends keyof RootStackParamList
> extends NavigationProps<ScreenName> {
	targetIdentity: LegacyAccount;
}

export interface NavigationAccountScannerProps<
	ScreenName extends keyof RootStackParamList
> extends NavigationProps<ScreenName> {
	scannerStore: ScannerContextType;
	accountsStore: AccountsContextType;
}

export interface NavigationScannerProps<
	ScreenName extends keyof RootStackParamList
> extends NavigationProps<ScreenName> {}
