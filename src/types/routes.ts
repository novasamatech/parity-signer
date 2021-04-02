export type RootStackParamList = {
	Wallet: undefined;
	AddNetwork: { isNew: boolean } | undefined;
	ShowRecoveryPhrase: { isNew: true } | { isNew: false; seedPhrase: string };
	RenameWallet: { identity };
	DeleteWallet: { identity };
	CreateWallet: { isRecover: boolean } | undefined;
	Settings: undefined;
	MessageDetails: undefined;
	Empty: undefined;
	ReceiveBalance: { path: string };
	SendBalance: { path: string };
	Security: undefined;
	SignTransaction: { isScanningNetworkSpec: true } | undefined;
	SignTransactionFinish: undefined;
	TxDetails: undefined;
};
