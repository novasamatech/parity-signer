package io.parity.signer

import android.util.Log
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import io.parity.signer.alerts.Confirm
import io.parity.signer.alerts.ErrorModal
import io.parity.signer.alerts.ShieldAlert
import io.parity.signer.components.Documents
import io.parity.signer.modals.*
import io.parity.signer.models.*
import io.parity.signer.screens.*
import io.parity.signer.uniffi.*

@Composable
fun ScreenSelector(
	screenData: ScreenData,
	alertState: State<AlertData?>,
	progress: State<Float?>,
	captured: State<Int?>,
	total: State<Int?>,
	button: (Action, String, String) -> Unit,
	signerDataModel: SignerDataModel
) {
	val button1: (Action) -> Unit = { action -> button(action, "", "") }
	val button2: (Action, String) -> Unit =
		{ action, details -> button(action, details, "") }
	when (screenData) {
		is ScreenData.DeriveKey -> NewAddressScreen(
			screenData.f,
			button = button2,
			addKey = signerDataModel::addKey,
			dbName = signerDataModel.dbName,
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
			alertState.value
		)
		is ScreenData.Log -> HistoryScreen(screenData.f, button2)
		is ScreenData.LogDetails -> LogDetails(screenData.f)
		is ScreenData.ManageNetworks -> ManageNetworks(
			screenData.f,
			button2,
		)
		is ScreenData.NNetworkDetails -> NetworkDetails(
			screenData.f,//default fallback
			button2
		)
		is ScreenData.NewSeed -> NewSeedScreen(
			screenData.f,
			signerDataModel::pushButton,
			signerDataModel = signerDataModel
		)
		is ScreenData.RecoverSeedName -> RecoverSeedName(
			screenData.f,
			signerDataModel::pushButton,
			signerDataModel = signerDataModel
		)
		is ScreenData.RecoverSeedPhrase -> RecoverSeedPhrase(
			screenData.f,
			signerDataModel::pushButton,
			signerDataModel = signerDataModel
		)
		ScreenData.Scan -> ScanScreen(
			progress = progress,
			captured = captured,
			total = total,
			button = signerDataModel::pushButton,
			signerDataModel = signerDataModel,
		)
		is ScreenData.SeedSelector -> SeedManager(
			screenData.f,
			signerDataModel = signerDataModel
		)
		is ScreenData.SelectSeedForBackup -> SelectSeedForBackup(
			screenData.f,
			signerDataModel = signerDataModel
		)
		is ScreenData.Settings -> SettingsScreen(
			screenData.f,
			signerDataModel = signerDataModel
		)
		is ScreenData.SignSufficientCrypto -> SignSufficientCrypto(
			screenData.f,
			signerDataModel = signerDataModel
		)
		is ScreenData.Transaction -> TransactionPreview(
			screenData.f,
			signerDataModel::pushButton,
			signerDataModel = signerDataModel
		)
		is ScreenData.VVerifier -> VerifierScreen(
			screenData.f,
			signerDataModel
		)
	}
}

@Composable
fun ModalSelector(
	modalData: ModalData?,
	alertState: AlertData?,
	button: (Action, String, String) -> Unit,
	signerDataModel: SignerDataModel
) {
	Log.w("SIGNER_RUST_LOG", "modal $modalData")
	Log.w("SIGNER_RUST_LOG", "alert $alertState")
	val button1: (Action) -> Unit = { action -> button(action, "", "") }
	val button2: (Action, String) -> Unit =
		{ action, details -> button(action, details, "") }
	val seedButton: (String) -> Unit =
		{ seed -> button(Action.GO_FORWARD, seed, "") }
	when (modalData) {
		is ModalData.NewSeedMenu -> when (alertState) {
			is AlertData.Shield -> NewSeedMenu(alertState.f, button1)
			else -> NewSeedMenu(null, button1)
		}
		is ModalData.SeedMenu -> SeedMenu(
			modalData.f,
			alertState,
			button1,
			removeSeed = seedButton
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
		is ModalData.NetworkDetailsMenu -> NetworkDetailsMenu(signerDataModel = signerDataModel)
		is ModalData.ManageMetadata -> {
			ManageMetadata(modalData.f, signerDataModel = signerDataModel)
		}
		is ModalData.SufficientCryptoReady -> SufficientCryptoReady(
			modalData.f,
		)
		is ModalData.KeyDetailsAction -> KeyDetailsAction(signerDataModel = signerDataModel)
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
	button: (Action, String, String) -> Unit,
) {
	val button1: (Action) -> Unit = { action -> button(action, "", "") }

	val ackWarning: () -> Unit = { button1(Action.GO_BACK) }
	when (alert) {
		AlertData.Confirm -> Confirm(button = button1)
		is AlertData.ErrorData -> ErrorModal(
			error = alert.f,
			button = button1
		)
		is AlertData.Shield -> ShieldAlert(
			alert.f,
			button = button1,
			acknowledgeWarning = ackWarning
		)
		null -> {}
	}
}

enum class OnBoardingState {
	InProgress,
	No,
	Yes;
}
