package io.parity.signer

import androidx.camera.core.ImageProxy
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import com.google.mlkit.vision.barcode.BarcodeScanner
import io.parity.signer.alerts.Confirm
import io.parity.signer.alerts.ErrorModal
import io.parity.signer.alerts.ShieldAlert
import io.parity.signer.components.Documents
import io.parity.signer.components.SeedBoxStatus
import io.parity.signer.modals.*
import io.parity.signer.models.*
import io.parity.signer.screens.*
import io.parity.signer.uniffi.*

@Composable
fun ScreenSelector(
	screenData: ScreenData,
	alertState: State<AlertState?>,
	progress: State<Float?>,
	captured: State<Int?>,
	total: State<Int?>,
	seedNames: Array<String>,
	isStrongBoxProtected: () -> Boolean,
	addKey: (String, String) -> Unit,
	checkPath: (String, String, String) -> DerivationCheck,
	increment: (Int, String) -> Unit,
	addSeed: (String, String, Boolean) -> Unit,
	handleCameraPermissions: () -> Unit,
	processFrame: (BarcodeScanner, ImageProxy) -> Unit,
	resetScanValues: () -> Unit,
	getAppVersion: () -> String,
	wipeToFactory: () -> Unit,
	signSufficientCrypto: (String, String) -> Unit,
	signTransaction: (String, String) -> Unit,
	wipeToJailbreak: () -> Unit,
	button: (Action, String, String) -> Unit
) {
	val button1: (Action) -> Unit = { action -> button(action, "", "") }
	val button2: (Action, String) -> Unit =
		{ action, details -> button(action, details, "") }

	when (screenData) {
		is ScreenData.DeriveKey -> NewAddressScreen(
			screenData.f,
			button = button2,
			addKey = addKey,
			checkPath = checkPath,
		)
		ScreenData.Documents -> Documents()
		is ScreenData.KeyDetails -> ExportPublicKey(screenData.f)
		is ScreenData.KeyDetailsMulti -> KeyDetailsMulti(
			screenData.f,
			button1,
		)
		is ScreenData.Keys -> KeyManager(
			button = button2,
			increment,
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
			button2,
			seedNames
		)
		is ScreenData.RecoverSeedName -> RecoverSeedName(
			screenData.f,
			button2,
			seedNames
		)
		is ScreenData.RecoverSeedPhrase -> RecoverSeedPhrase(
			recoverSeedPhrase = screenData.f,
			button = button2,
			addSeed = addSeed
		)
		ScreenData.Scan -> ScanScreen(
			progress = progress,
			captured = captured,
			total = total,
			button = button,
			handleCameraPermissions = handleCameraPermissions,
			processFrame = processFrame,
			resetScanValues = resetScanValues,
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
			isStrongBoxProtected = isStrongBoxProtected,
			getAppVersion = getAppVersion,
			wipeToFactory = wipeToFactory,
			alertState = alertState,
		)
		is ScreenData.SignSufficientCrypto -> SignSufficientCrypto(
			screenData.f,
			signSufficientCrypto
		)
		is ScreenData.Transaction -> TransactionPreview(
			screenData.f,
			button,
			signTransaction
		)
		is ScreenData.VVerifier -> VerifierScreen(
			screenData.f,
			wipeToJailbreak
		)
		is ScreenData.SignatureReady -> SignatureReady(
			screenData.f,
			button
		)
	}
}

@Composable
fun ModalSelector(
	modalData: ModalData?,
	alertState: State<AlertState?>,
	removeSeed: (String) -> Unit,
	getSeedForBackup: (String, (String) -> Unit, (SeedBoxStatus) -> Unit) -> Unit,
	addKey: (String, String) -> Unit,
	addSeed: (String, String, Boolean) -> Unit,
	selectSeed: (String) -> Unit,
	button: (Action, String, String) -> Unit
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
			removeSeed
		)
		is ModalData.NetworkSelector -> NetworkSelector(
			modalData.f,
			button2
		)
		is ModalData.Backup -> SeedBackup(
			modalData.f,
			getSeedForBackup = getSeedForBackup
		)
		is ModalData.PasswordConfirm -> PasswordConfirm(
			modalData.f,
			addKey = addKey
		)
		is ModalData.EnterPassword -> EnterPassword(
			modalData.f,
			button2,
		)
		is ModalData.LogRight -> LogMenu(
			modalData.f,
			button1
		)
		is ModalData.NetworkDetailsMenu -> NetworkDetailsMenu(
			button1
		)
		is ModalData.ManageMetadata -> {
			ManageMetadata(
				modalData.f,
				button1
			)
		}
		is ModalData.SufficientCryptoReady -> SufficientCryptoReady(
			modalData.f,
		)
		is ModalData.KeyDetailsAction -> KeyDetailsAction(
			button1
		)
		is ModalData.TypesInfo -> TypesInfo(
			modalData.f,
			button1
		)
		is ModalData.NewSeedBackup -> NewSeedBackup(
			modalData.f,
			addSeed
		)
		is ModalData.LogComment -> LogComment(
			button2
		)
		is ModalData.SelectSeed -> {
			SelectSeed(modalData.f, selectSeed)
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
