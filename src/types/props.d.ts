import { NavigationStackScreenProps } from 'react-navigation-stack';
import { NavigationInjectedProps } from 'react-navigation';

export interface NavigationProps<Params, ScreenProps> {
	navigation: NavigationStackScreenProps<Params, ScreenProps>;
}

export type ScreenProps = NavigationInjectedProps;
