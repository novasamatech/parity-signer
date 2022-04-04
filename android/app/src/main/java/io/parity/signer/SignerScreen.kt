package io.parity.signer

import androidx.compose.animation.ExperimentalAnimationApi
import androidx.compose.material.ExperimentalMaterialApi
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.livedata.observeAsState
import io.parity.signer.alerts.Confirm
import io.parity.signer.alerts.ErrorModal
import io.parity.signer.alerts.ShieldAlert
import io.parity.signer.components.Documents
import io.parity.signer.modals.*
import io.parity.signer.models.*
import io.parity.signer.screens.*
import org.json.JSONObject

@ExperimentalAnimationApi
@ExperimentalMaterialApi
@Composable
fun ScreenSelector(screen: SignerScreen?, signerDataModel: SignerDataModel) {
	val screenData by signerDataModel.screenData.observeAsState()
	val alertState by signerDataModel.alertState.observeAsState()

	when (screen) {
		SignerScreen.Scan -> {
			ScanScreen(
				signerDataModel::handleCameraPermissions,
				signerDataModel::processFrame,
				signerDataModel.progress.observeAsState(),
				signerDataModel.captured.observeAsState(),
				signerDataModel.total.observeAsState(),
				signerDataModel::resetScan
			)
		}
		SignerScreen.Keys -> {
			KeyManager(
				signerDataModel::pushButton,
				signerDataModel::increment,
				screenData ?: JSONObject(),
				alertState
			)
		}
		SignerScreen.Settings -> {
			SettingsScreen(
				screenData ?: JSONObject(),
				signerDataModel::pushButton,
				alertState,
				signerDataModel.isStrongBoxProtected().toString(),
				signerDataModel.getAppVersion(),
				signerDataModel::wipeToFactory
			)
		}
		SignerScreen.Log -> {
			HistoryScreen(
				screenData ?: JSONObject(),
				signerDataModel::pushButton
			)
		}
		SignerScreen.LogDetails -> LogDetails(screenData ?: JSONObject())
		SignerScreen.Transaction -> {
			TransactionPreview(
				screenData ?: JSONObject(),
				signerDataModel::pushButton,
				signerDataModel::signTransaction
			)
		}
		SignerScreen.SeedSelector -> {
			SeedManager(
				screenData ?: JSONObject(),
				signerDataModel::pushButton
			)
		}
		SignerScreen.KeyDetails -> {
			ExportPublicKey(
				screenData ?: JSONObject()
			)
		}
		SignerScreen.NewSeed -> {
			NewSeedScreen(
				screenData ?: JSONObject(),
				signerDataModel.seedNames.value ?: arrayOf(),
				signerDataModel::pushButton
			)
		}
		SignerScreen.RecoverSeedName -> {
			RecoverSeedName(
				screenData ?: JSONObject(),
				signerDataModel.seedNames.value ?: arrayOf(),
				signerDataModel::pushButton
			)
		}
		SignerScreen.RecoverSeedPhrase -> {
			RecoverSeedPhrase(
				screenData ?: JSONObject(),
				signerDataModel::pushButton,
				signerDataModel::addSeed
			)
		}
		SignerScreen.DeriveKey -> {
			NewAddressScreen(
				screenData ?: JSONObject(),
				signerDataModel::pushButton,
				signerDataModel::pathCheck,
				signerDataModel::addKey
			)
		}
		SignerScreen.Verifier -> VerifierScreen(
			screenData ?: JSONObject(),
			signerDataModel::wipeToJailbreak
		)
		null -> WaitingScreen()
		SignerScreen.ManageNetworks -> ManageNetworks(
			screenData ?: JSONObject(),
			signerDataModel::pushButton
		)
		SignerScreen.NetworkDetails -> NetworkDetails(
			screenData ?: JSONObject(),
			signerDataModel::pushButton
		)
		SignerScreen.SignSufficientCrypto -> SignSufficientCrypto(
			screenData ?: JSONObject(),
			signerDataModel::signSufficientCrypto
		)
		SignerScreen.SelectSeedForBackup -> SelectSeedForBackup(
			screenData ?: JSONObject(),
			signerDataModel::pushButton
		)
		SignerScreen.Documents -> Documents()
		SignerScreen.KeyDetailsMultiSelect -> KeyDetailsMulti(
			screenData ?: JSONObject(),
			signerDataModel::pushButton
		)
	}
}

@Composable
fun ModalSelector(modal: SignerModal, signerDataModel: SignerDataModel) {
	val modalData = signerDataModel.modalData.observeAsState()
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
		SignerModal.ManageMetadata -> ManageMetadata(signerDataModel = signerDataModel)
		SignerModal.SufficientCryptoReady -> SufficientCryptoReady(signerDataModel = signerDataModel)
		SignerModal.KeyDetailsAction -> KeyDetailsAction(signerDataModel = signerDataModel)
		SignerModal.TypesInfo -> TypesInfo(signerDataModel = signerDataModel)
		SignerModal.NewSeedBackup -> NewSeedBackup(signerDataModel = signerDataModel)
		SignerModal.LogComment -> LogComment(signerDataModel = signerDataModel)
		SignerModal.SelectSeed -> SelectSeed(signerDataModel = signerDataModel)
	}
}

@Composable
fun AlertSelector(alert: SignerAlert, signerDataModel: SignerDataModel) {
	when (alert) {
		SignerAlert.Empty -> {}
		SignerAlert.Error -> ErrorModal(
			error = signerDataModel.alertData.value?.optString(
				"error"
			) ?: "unknown error",
			signerDataModel::pushButton
		)
		SignerAlert.Shield -> ShieldAlert(
			signerDataModel.alertState.observeAsState(),
			signerDataModel::pushButton,
			signerDataModel::acknowledgeWarning
		)
		SignerAlert.Confirm -> Confirm(signerDataModel::pushButton)
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
	Documents,
	KeyDetailsMultiSelect;
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
	LogComment,
	SelectSeed;
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
