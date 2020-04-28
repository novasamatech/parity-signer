import { Identity } from 'types/identityTypes';
import { SeedRefClass } from 'utils/native';

export type RootStackParamList = {
	About: undefined;
	AccountDetails: undefined;
	AccountEdit: undefined;
	AccountPin: { isNew: boolean } | undefined;
	AccountNew: undefined;
	Main: { isNew: boolean } | undefined;
	AccountUnlockAndSign: { next: string };
	AccountUnlock: { next: string; onDelete: () => any };
	IdentityBackup: { isNew: true } | { isNew: false; seedPhrase: string };
	IdentityManagement: undefined;
	IdentityNew: { isRecover: boolean } | undefined;
	MessageDetails: undefined;
	Empty: undefined;
	LegacyAccountBackup:
		| {
				isNew: boolean;
		  }
		| undefined;
	LegacyAccountList: undefined;
	LegacyNetworkChooser: undefined;
	PathDerivation: { parentPath: string };
	PathDetails: { path: string };
	PathManagement: { path: string };
	PathsList: { networkKey: string };
	PinNew: { resolve: (pin: string) => void };
	PinUnlock:
		| {
				identity?: Identity;
				resolve: (seedPhrase: string) => void;
				shouldReturnSeed: true;
		  }
		| {
				identity?: Identity;
				resolve: (seedRef: SeedRefClass) => void;
				shouldReturnSeed: false;
		  };
	PinUnlockWithPassword:
		| {
				identity?: Identity;
				isSeedRefValid: true;
				resolve: (password: string) => void;
		  }
		| {
				identity?: Identity;
				isSeedRefValid: false;
				resolve: ([password, seedRef]: [string, SeedRefClass]) => void;
		  };
	PrivacyPolicy: undefined;
	QrScanner: undefined;
	Security: undefined;
	SignedMessage: undefined;
	SignedTx: undefined;
	TermsAndConditions: undefined;
	TxDetails: undefined;
};
