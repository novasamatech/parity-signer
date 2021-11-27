package io.parity.signer

/**
 * All screens metadata for navigation
 */
enum class SignerScreen() {
	Scan,
	Keys,
	Settings,
	Log;
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
