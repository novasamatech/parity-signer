import { Identity } from 'types/identityTypes';

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
	RootSeedNew: { isRecover: boolean } | undefined;
	MessageDetails: undefined;
	Empty: undefined;
	LegacyAccountBackup:
		| {
				isNew: boolean;
		  }
		| undefined;
	LegacyAccountList: undefined;
	LegacyNetworkChooser: undefined;
	NetworkDetails: { pathId: string };
	NetworkSettings: undefined;
	MetadataManagement: { pathId: string };
	MetadataSaving: { metadata: string };
	FullMetadata: { pathId: string };
	FullMetadataViewer: { pathId: string };
	PathDerivation: { parentPath: string };
	PathDetails: { path: string };
	PathManagement: { path: string };
	PathSecret: { path: string; password?: string };
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
				resolve: () => void;
				shouldReturnSeed: false;
		  };
	PinUnlockWithPassword: {
		identity?: Identity;
		isSeedRefValid: boolean;
		resolve: (password: string) => void;
	};
	PrivacyPolicy: undefined;
	QrScanner:
		| undefined
		| {
				isScanningNetworkSpec: true;
		  };
	FastQrScanner: undefined;
	Security: undefined;
	DetailsMessage: undefined;
	SignedMessage: undefined;
	DetailsTx: { payload: string };
	SignedTx: { payload: string };
	TermsAndConditions: undefined;
	TxDetails: undefined;
};
