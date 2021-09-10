package io.parity.signer

import androidx.compose.runtime.Composable
import io.parity.signer.screens.HomeScreen

/**
 * All screens metadata for navigation
 */
enum class SignerScreen() {
	Home, Keys, Settings;
}

enum class TransactionState {
	None, Parsing, Preview, Password, Signed;
}

enum class KeyManagerModal {
	None, NewSeed, NewKey, ShowKey, SeedBackup, KeyDeleteConfirm;
}

enum class SettingsModal {
	None, History;
}

enum class OnBoardingState {
	InProgress, No, Yes;
}
