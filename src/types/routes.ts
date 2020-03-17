import { Identity } from 'types/identityTypes';

export type RootStackParamList = {
	// Home: undefined;
	// AccountNetworkChooser: { userId: string };
	// Feed: { sort: 'latest' | 'top' } | undefined;
	About: undefined;
	AccountDetails: undefined;
	AccountEdit: undefined;
	AccountPin: { isNew: boolean };
	AccountNew: undefined;
	AccountNetworkChooser: { isNew: boolean };
	AccountUnlockAndSign: { next: string };
	AccountUnlock: { next: string; onDelete: () => any };
	IdentityBackup: { isNew: boolean };
	IdentityManagement: undefined;
	IdentityNew: { isRecover: boolean };
	IdentityPin: {
		isUnlock?: boolean;
		isNew?: boolean;
		identity?: Identity;
		resolve: (returnValue: string) => void;
	};
	MessageDetails: undefined;
	Empty: undefined;
	LegacyAccountBackup: {
		isNew: boolean;
	};
	LegacyAccountList: undefined;
	LegacyNetworkChooser: undefined;
	PathDerivation: { parentPath: string };
	PathDetails: { path?: string };
	PathManagement: { path?: string };
	PathsList: { networkKey?: string };
	PrivacyPolicy: undefined;
	QrScanner: undefined;
	Security: undefined;
	SignedMessage: undefined;
	SignedTx: undefined;
	TermsAndConditions: undefined;
	TxDetails: undefined;
};
