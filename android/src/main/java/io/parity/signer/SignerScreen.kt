package io.parity.signer

import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import io.parity.signer.alerts.Confirm
import io.parity.signer.alerts.ErrorModal
import io.parity.signer.alerts.ShieldAlert
import io.parity.signer.components.Documents
import io.parity.signer.modals.*
import io.parity.signer.models.*
import io.parity.signer.screens.*
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.AlertData
import io.parity.signer.uniffi.ModalData
import io.parity.signer.uniffi.ScreenData

@Composable
fun ScreenSelector(
	screenData: ScreenData,
	alertState: State<AlertState?>,
	progress: State<Float?>,
	captured: State<Int?>,
	total: State<Int?>,
	button: (Action, String, String) -> Unit,
	signerDataModel: SignerDataModel
) {
	val button1: (Action) -> Unit = { action -> button(action, "", "") }
	val button2: (Action, String) -> Unit =
		{ action, details -> button(action, details, "") }
	val seedNames = signerDataModel.seedNames.value ?: emptyArray()

	when (screenData) {
		is ScreenData.DeriveKey -> NewAddressScreen(
			screenData.f,
			button = button2,
			addKey = signerDataModel::addKey,
			checkPath = signerDataModel::checkPath,
		)
		ScreenData.Documents -> Documents()
		is ScreenData.KeyDetails -> ExportPublicKey(screenData.f)
		is ScreenData.KeyDetailsMulti -> KeyDetailsMulti(
			screenData.f,
			button1,
		)
		is ScreenData.Keys -> KeyManager(
			button = button2,
			signerDataModel::increment,
			screenData.f,
			alertState
		)
		is ScreenData.Log -> HistoryScreen(screenData.f, button2)
		is ScreenData.LogDetails -> LogDetails(screenData.f)
		is ScreenData.ManageNetworks -> ManageNetworks(
			screenData.f,
			button2,
		)
		is ScreenData.NNetworkDetails -> NetworkDetails(
			screenData.f,
			button2
		)
		is ScreenData.NewSeed -> NewSeedScreen(
			screenData.f,
			signerDataModel::pushButton,
			seedNames
		)
		is ScreenData.RecoverSeedName -> RecoverSeedName(
			screenData.f,
			signerDataModel::pushButton,
			seedNames
		)
		is ScreenData.RecoverSeedPhrase -> RecoverSeedPhrase(
			recoverSeedPhrase = screenData.f,
			button = signerDataModel::pushButton,
			addSeed = signerDataModel::addSeed
		)
		ScreenData.Scan -> ScanScreen(
			progress = progress,
			captured = captured,
			total = total,
			button = signerDataModel::pushButton,
			handleCameraPermissions = signerDataModel::handleCameraPermissions,
			processFrame = signerDataModel::processFrame,
			resetScanValues = signerDataModel::resetScanValues,
		)
		is ScreenData.SeedSelector -> SeedManager(
			screenData.f,
			button2
		)
		is ScreenData.SelectSeedForBackup -> SelectSeedForBackup(
			screenData.f,
			button2
		)
		is ScreenData.Settings -> SettingsScreen(
			screenData.f,
			button1 = button1,
			isStrongBoxProtected = signerDataModel::isStrongBoxProtected,
			getAppVersion = signerDataModel::getAppVersion,
			wipeToFactory = signerDataModel::wipeToFactory,
			alertState = alertState,
		)
		is ScreenData.SignSufficientCrypto -> SignSufficientCrypto(
			screenData.f,
			signerDataModel::signSufficientCrypto
		)
		is ScreenData.Transaction -> TransactionPreview(
			screenData.f,
			signerDataModel::pushButton,
			signerDataModel::signTransaction
		)
		is ScreenData.VVerifier -> VerifierScreen(
			screenData.f,
			signerDataModel::wipeToJailbreak
		)
	}
}

@Composable
fun ModalSelector(
	modalData: ModalData?,
	alertState: State<AlertState?>,
	button: (Action, String, String) -> Unit,
	signerDataModel: SignerDataModel
) {
	val button1: (Action) -> Unit = { action -> button(action, "", "") }
	val button2: (Action, String) -> Unit =
		{ action, details -> button(action, details, "") }
	when (modalData) {
		is ModalData.NewSeedMenu ->
			NewSeedMenu(
				alertState = alertState,
				button = button1
			)
		is ModalData.SeedMenu -> SeedMenu(
			modalData.f,
			alertState,
			button1,
			signerDataModel::removeSeed
		)
		is ModalData.NetworkSelector -> NetworkSelector(
			modalData.f,
			button2
		)
		is ModalData.Backup -> SeedBackup(
			modalData.f,
			getSeedForBackup = signerDataModel::getSeedForBackup
		)
		is ModalData.PasswordConfirm -> PasswordConfirm(
			modalData.f,
			signerDataModel = signerDataModel
		)
		is ModalData.SignatureReady -> SignatureReady(
			modalData.f,
			signerDataModel = signerDataModel
		)
		is ModalData.EnterPassword -> EnterPassword(
			modalData.f,
			button2,
		)
		is ModalData.LogRight -> LogMenu(
			modalData.f,
			signerDataModel = signerDataModel
		)
		is ModalData.NetworkDetailsMenu -> NetworkDetailsMenu(
			signerDataModel = signerDataModel
		)
		is ModalData.ManageMetadata -> {
			ManageMetadata(modalData.f, signerDataModel = signerDataModel)
		}
		is ModalData.SufficientCryptoReady -> SufficientCryptoReady(
			modalData.f,
		)
		is ModalData.KeyDetailsAction -> KeyDetailsAction(
			signerDataModel = signerDataModel
		)
		is ModalData.TypesInfo -> TypesInfo(
			modalData.f,
			signerDataModel = signerDataModel
		)
		is ModalData.NewSeedBackup -> NewSeedBackup(
			modalData.f,
			signerDataModel = signerDataModel
		)
		is ModalData.LogComment -> LogComment(signerDataModel = signerDataModel)
		is ModalData.SelectSeed -> {
			SelectSeed(modalData.f, signerDataModel = signerDataModel)
		}
		null -> {}
	}
}

@Composable
fun AlertSelector(
	alert: AlertData?,
	alertState: State<AlertState?>,
	button: (Action, String, String) -> Unit,
	acknowledgeWarning: () -> Unit
) {
	val button1: (Action) -> Unit = { action -> button(action, "", "") }

	when (alert) {
		AlertData.Confirm -> Confirm(button = button1)
		is AlertData.ErrorData -> ErrorModal(
			error = alert.f,
			button = button1
		)
		is AlertData.Shield -> ShieldAlert(
			// alert.f, // TODO: use this instead
			alertState = alertState,
			button = button1,
			acknowledgeWarning = acknowledgeWarning
		)
		null -> {}
	}
}

enum class OnBoardingState {
	InProgress,
	No,
	Yes;
}
