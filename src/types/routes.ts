import { Identity } from 'types/identityTypes';

export type RootStackParamList = {
	Main: { isNew: boolean } | undefined;
	IdentityBackup: { isNew: true } | { isNew: false; seedPhrase: string };
	IdentityManagement: undefined;
	IdentityNew: { isRecover: boolean } | undefined;
	IdentitySwitch: undefined;
	MessageDetails: undefined;
	Empty: undefined;
	PathDetails: { path: string };
	QrScanner:
		| undefined
		| {
				isScanningNetworkSpec: true;
		  };
	Security: undefined;
	SignedMessage: undefined;
	SignedTx: undefined;
	TxDetails: undefined;
};
