import {Identity} from 'types/identityTypes';

export type RootStackParamList = {
	// Home: undefined;
	// AccountNetworkChooser: { userId: string };
	// Feed: { sort: 'latest' | 'top' } | undefined;

	AccountNetworkChooser: {isNew: boolean };
	AccountUnlockAndSign: {next: string};
	AccountUnlock: {next: string, onDelete: ()=> any}
	IdentitiesSwitch: { isSwitchOpen?: boolean }; //TODO check if it works
	IdentityBackup: {isNew: boolean},
	IdentityNew: {isRecover: boolean},
	IdentityPin: {
		isUnlock?: boolean;
		isNew?: boolean;
		identity?: Identity;
		resolve: (returnValue: string) => Promise<string>;
	},
	LegacyAccountBackup: {
		isNew: boolean
	},
	PathDerivation: { parentPath: string },
	PathDetails: { path?: string },
	PathManagement: {path?: string},
	PathsList: {networkKey?: string},
	TermsAndConditions: { disableButtons: boolean }
};
