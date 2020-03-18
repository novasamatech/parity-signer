import { Identity } from 'types/identityTypes';

export type RootStackParamList = {
	About: undefined;
	AccountDetails: undefined;
	AccountEdit: undefined;
	AccountPin: { isNew: boolean } | undefined;
	AccountNew: undefined;
	AccountNetworkChooser: { isNew: boolean } | undefined;
	AccountUnlockAndSign: { next: string };
	AccountUnlock: { next: string; onDelete: () => any };
	IdentityBackup: { isNew: true } | { isNew: false; seedPhrase: string };
	IdentityManagement: undefined;
	IdentityNew: { isRecover: boolean } | undefined;
	IdentityPin: {
		isUnlock?: boolean;
		isNew?: boolean;
		identity?: Identity;
		resolve: (returnValue: string) => void;
	};
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
	PrivacyPolicy: undefined;
	QrScanner: undefined;
	Security: undefined;
	SignedMessage: undefined;
	SignedTx: undefined;
	TermsAndConditions: undefined;
	TxDetails: undefined;
};
