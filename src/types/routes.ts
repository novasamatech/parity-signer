export type RootStackParamList = {
	Main: undefined;
	AddNetwork: { isNew: boolean } | undefined;
	ShowRecoveryPhrase: { isNew: true } | { isNew: false; seedPhrase: string };
	RenameWallet: { identity };
	DeleteWallet: { identity };
	CreateWallet: { isRecover: boolean } | undefined;
	Settings: undefined;
	MessageDetails: undefined;
	Empty: undefined;
	AddToPolkadotJs: { path: string };
	SignTx:
		| undefined
		| {
				isScanningNetworkSpec: true;
		  };
	Security: undefined;
	SignedMessage: undefined;
	SignedTx: undefined;
	TxDetails: undefined;
};
