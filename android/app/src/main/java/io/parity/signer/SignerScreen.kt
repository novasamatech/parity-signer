package io.parity.signer

import androidx.compose.animation.ExperimentalAnimationApi
import androidx.compose.material.ExperimentalMaterialApi
import androidx.compose.runtime.Composable
import io.parity.signer.components.Documents
import io.parity.signer.modals.*
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton
import io.parity.signer.screens.*

@ExperimentalAnimationApi
@ExperimentalMaterialApi
@Composable
fun ScreenSelector(screen: SignerScreen?, signerDataModel: SignerDataModel) {
	when (screen) {
		SignerScreen.Scan -> {
			ScanScreen(
				signerDataModel = signerDataModel
			)
		}
		SignerScreen.Keys -> {
			KeyManager(signerDataModel = signerDataModel)
		}
		SignerScreen.Settings -> {
			SettingsScreen(signerDataModel = signerDataModel)
		}
		SignerScreen.Log -> {
			HistoryScreen(signerDataModel = signerDataModel)
		}
		SignerScreen.LogDetails -> LogDetails(signerDataModel = signerDataModel)
		SignerScreen.Transaction -> {
			TransactionPreview(
				signerDataModel::pushButton,
				signerDataModel = signerDataModel
			)
		}
		SignerScreen.SeedSelector -> {
			SeedManager(signerDataModel = signerDataModel)
		}
		SignerScreen.KeyDetails -> {
			ExportPublicKey(signerDataModel = signerDataModel)
		}
		SignerScreen.NewSeed -> {
			NewSeedScreen(
				signerDataModel::pushButton,
				signerDataModel = signerDataModel
			)
		}
		SignerScreen.RecoverSeedName -> {
			RecoverSeedName(
				signerDataModel::pushButton,
				signerDataModel = signerDataModel
			)
		}
		SignerScreen.RecoverSeedPhrase -> {
			RecoverSeedPhrase(
				signerDataModel::pushButton,
				signerDataModel = signerDataModel
			)
		}
		SignerScreen.DeriveKey -> {
			NewAddressScreen(signerDataModel = signerDataModel, increment = false)
		}
		SignerScreen.Verifier -> VerifierScreen(signerDataModel)
		null -> WaitingScreen()
		SignerScreen.ManageNetworks -> ManageNetworks(signerDataModel = signerDataModel)
		SignerScreen.NetworkDetails -> NetworkDetails(signerDataModel = signerDataModel)
		SignerScreen.SignSufficientCrypto -> SignSufficientCrypto(signerDataModel = signerDataModel)
		SignerScreen.SelectSeedForBackup -> SelectSeedForBackup(signerDataModel = signerDataModel)
		SignerScreen.Documents -> Documents()
	}
}

@Composable
fun ModalSelector(modal: SignerModal, signerDataModel: SignerDataModel) {
	when (modal) {
		SignerModal.Empty -> {}
		SignerModal.NewSeedMenu -> NewSeedMenu(signerDataModel = signerDataModel)
		SignerModal.SeedMenu -> SeedMenu(signerDataModel = signerDataModel)
		SignerModal.NetworkSelector -> NetworkSelector(signerDataModel = signerDataModel)
		SignerModal.Backup -> SeedBackup(signerDataModel = signerDataModel)
		SignerModal.PasswordConfirm -> PasswordConfirm(signerDataModel = signerDataModel)
		SignerModal.SignatureReady -> SignatureReady(signerDataModel = signerDataModel)
		SignerModal.EnterPassword -> EnterPassword(signerDataModel = signerDataModel)
		SignerModal.LogRight -> LogMenu(signerDataModel = signerDataModel)
		SignerModal.NetworkDetailsMenu -> NetworkDetailsMenu(signerDataModel = signerDataModel)
		SignerModal.ManageMetadata -> TODO()
		SignerModal.SufficientCryptoReady -> TODO()
		SignerModal.KeyDetailsAction -> TODO()
		SignerModal.TypesInfo -> TODO()
		SignerModal.NewSeedBackup -> NewSeedBackup(signerDataModel = signerDataModel)
		SignerModal.LogComment -> TODO()
	}
}

@Composable
fun AlertSelector(alert: SignerAlert, signerDataModel: SignerDataModel) {
	when (alert) {
		SignerAlert.Empty -> {}
		SignerAlert.Error -> ErrorModal(
			error = signerDataModel.alertData.value?.optString(
				"error"
			) ?: "unknown error", signerDataModel = signerDataModel
		)
		SignerAlert.Shield -> ShieldAlert(signerDataModel)
		SignerAlert.Confirm -> TODO()
	}
}


/**
 * All screens metadata for navigation
 */
enum class SignerScreen {
	Scan,
	Keys,
	Settings,
	Log,
	LogDetails,
	Transaction,
	SeedSelector,
	KeyDetails,
	NewSeed,
	RecoverSeedName,
	RecoverSeedPhrase,
	DeriveKey,
	Verifier,
	ManageNetworks,
	NetworkDetails,
	SignSufficientCrypto,
	SelectSeedForBackup,
	Documents;
}

enum class SignerModal {
	Empty,
	NewSeedMenu,
	NetworkSelector,
	SeedMenu,
	Backup,
	PasswordConfirm,
	SignatureReady,
	EnterPassword,
	LogRight,
	NetworkDetailsMenu,
	ManageMetadata,
	SufficientCryptoReady,
	KeyDetailsAction,
	TypesInfo,
	NewSeedBackup,
	LogComment;
}

enum class SignerAlert {
	Empty,
	Error,
	Shield,
	Confirm;
}

enum class OnBoardingState {
	InProgress,
	No,
	Yes;
}

enum class ShieldAlert {
	None,
	Active,
	Past
}
