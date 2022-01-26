package io.parity.signer

import androidx.compose.animation.ExperimentalAnimationApi
import androidx.compose.material.ExperimentalMaterialApi
import androidx.compose.runtime.Composable
import io.parity.signer.modals.*
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton
import io.parity.signer.screens.KeyManager
import io.parity.signer.screens.RecoverSeedPhrase
import io.parity.signer.screens.ScanScreen
import io.parity.signer.screens.SettingsScreen

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
		SignerScreen.LogDetails -> TODO()
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
		SignerScreen.Verifier -> TODO()
		null -> WaitingScreen()
		SignerScreen.ManageNetworks -> TODO()
		SignerScreen.NetworkDetails -> TODO()
		SignerScreen.SignSufficientCrypto -> TODO()
		SignerScreen.SelectSeedForBackup -> TODO()
		SignerScreen.Documents -> TODO()
	}
}

@Composable
fun ModalSelector(modal: SignerModal, signerDataModel: SignerDataModel) {
	when (modal) {
		SignerModal.Empty -> {}
		SignerModal.NewSeedMenu -> NewSeedMenu(signerDataModel = signerDataModel)
		SignerModal.SeedMenu -> TODO()
		SignerModal.NetworkMenu -> TODO()
		SignerModal.Backup -> TODO()
		SignerModal.PasswordConfirm -> TODO()
		SignerModal.SignatureReady -> SignatureReady(signerDataModel = signerDataModel)
		SignerModal.EnterPassword -> TODO()
		SignerModal.LogRight -> TODO()
		SignerModal.NetworkDetailsMenu -> TODO()
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
			error = signerDataModel.screenData.value?.optString(
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
	NetworkMenu,
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
