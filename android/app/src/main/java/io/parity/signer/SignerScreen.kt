package io.parity.signer

/**
 * All screens metadata for navigation
 */
enum class SignerScreen() {
	Log,
	LogDetails,
	Scan,
	Transaction,
	SeedSelector,
	Keys,
	KeyDetails,
	Backup,
	NewSeed,
	RecoverSeedName,
	RecoverSeedPhrase,
	DeriveKey,
	Settings,
	Verifier,
	ManageNetwork;
}

enum class TransactionState {
	None,
	Parsing,
	Preview,
	Password,
	Signed;
}

enum class KeyManagerModal {
	None,
	NewSeedSelect,
	NewSeed,
	RestoreSeed,
	NewKey,
	ShowKey,
	SeedBackup,
	SeedDeleteConfirm,
	KeyDeleteConfirm,
	SeedSelector,
	NetworkManager,
	NetworkDetails,
	AllKeySelector;
}

enum class SettingsModal {
	None,
	History;
}

enum class OnBoardingState {
	InProgress,
	No,
	Yes;
}

enum class SignerAlert {
	None,
	Active,
	Past
}
