package io.parity.signer

import androidx.compose.animation.ExperimentalAnimationApi
import androidx.compose.material.ExperimentalMaterialApi
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
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
fun ScreenSelector(
	screen: SignerScreen?,
	screenData: State<JSONObject?>,
	shieldAlert: State<ShieldAlert?>,
	progress: State<Float?>,
	captured: State<Int?>,
	total: State<Int?>,
	button: (ButtonID, String, String) -> Unit,
	signerDataModel: SignerDataModel
) {
	when (screen) {
		SignerScreen.Scan -> {
			ScanScreen(
				signerDataModel::handleCameraPermissions,
				signerDataModel::processFrame,
				progress,
				captured,
				total,
				signerDataModel::resetScan
			)
		}
		SignerScreen.Keys -> {
			KeyManager(
				signerDataModel::pushButton,
				signerDataModel::increment,
				screenData.value ?: JSONObject(),
				shieldAlert.value
			)
		}
		SignerScreen.Settings -> {
			SettingsScreen(
				screenData.value ?: JSONObject(),
				signerDataModel::pushButton,
				shieldAlert.value,
				signerDataModel.isStrongBoxProtected().toString(),
				signerDataModel.getAppVersion(),
				signerDataModel::wipeToFactory
			)
		}
		SignerScreen.Log -> {
			HistoryScreen(
				screenData.value ?: JSONObject(),
				signerDataModel::pushButton
			)
		}
		SignerScreen.LogDetails -> LogDetails(screenData.value ?: JSONObject())
		SignerScreen.Transaction -> {
			TransactionPreview(
				screenData.value ?: JSONObject(),
				signerDataModel::pushButton,
				signerDataModel::signTransaction
			)
		}
		SignerScreen.SeedSelector -> {
			SeedManager(
				screenData.value ?: JSONObject(),
				signerDataModel::pushButton
			)
		}
		SignerScreen.KeyDetails -> {
			ExportPublicKey(
				screenData.value ?: JSONObject()
			)
		}
		SignerScreen.NewSeed -> {
			NewSeedScreen(
				screenData.value ?: JSONObject(),
				signerDataModel.seedNames.value ?: arrayOf(),
				signerDataModel::pushButton
			)
		}
		SignerScreen.RecoverSeedName -> {
			RecoverSeedName(
				screenData.value ?: JSONObject(),
				signerDataModel.seedNames.value ?: arrayOf(),
				signerDataModel::pushButton
			)
		}
		SignerScreen.RecoverSeedPhrase -> {
			RecoverSeedPhrase(
				screenData.value ?: JSONObject(),
				signerDataModel::pushButton,
				signerDataModel::addSeed
			)
		}
		SignerScreen.DeriveKey -> {
			NewAddressScreen(
				screenData.value ?: JSONObject(),
				signerDataModel::pushButton,
				signerDataModel::pathCheck,
				signerDataModel::addKey
			)
		}
		SignerScreen.Verifier -> VerifierScreen(
			screenData.value ?: JSONObject(),
			signerDataModel::wipeToJailbreak
		)
		null -> WaitingScreen()
		SignerScreen.ManageNetworks -> ManageNetworks(
			screenData.value ?: JSONObject(),
			signerDataModel::pushButton
		)
		SignerScreen.NetworkDetails -> NetworkDetails(
			screenData.value ?: JSONObject(),
			signerDataModel::pushButton
		)
		SignerScreen.SignSufficientCrypto -> SignSufficientCrypto(
			screenData.value ?: JSONObject(),
			signerDataModel::signSufficientCrypto
		)
		SignerScreen.SelectSeedForBackup -> SelectSeedForBackup(
			screenData.value ?: JSONObject(),
			signerDataModel::pushButton
		)
		SignerScreen.Documents -> Documents()
		SignerScreen.KeyDetailsMultiSelect -> KeyDetailsMulti(
			screenData.value ?: JSONObject(),
			signerDataModel::pushButton
		)
	}
}

@Composable
fun ModalSelector(modal: SignerModal, signerDataModel: SignerDataModel) {
	val modalData by signerDataModel.modalData.observeAsState()
	when (modal) {
		SignerModal.Empty -> {}
		SignerModal.NewSeedMenu -> NewSeedMenu(signerDataModel = signerDataModel)
		SignerModal.SeedMenu -> SeedMenu(signerDataModel = signerDataModel)
		SignerModal.NetworkSelector -> NetworkSelector(signerDataModel = signerDataModel)
		SignerModal.Backup -> SeedBackup(signerDataModel = signerDataModel)
		SignerModal.PasswordConfirm -> PasswordConfirm(signerDataModel = signerDataModel)
		SignerModal.SignatureReady -> SignatureReady(signerDataModel = signerDataModel)
		SignerModal.EnterPassword -> EnterPassword(
			modalData?: JSONObject(),
			signerDataModel::pushButton
		)
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
