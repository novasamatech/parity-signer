package io.parity.signer

import android.util.Log
import androidx.compose.animation.ExperimentalAnimationApi
import androidx.compose.material.ExperimentalMaterialApi
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.livedata.observeAsState
import androidx.lifecycle.LiveData
import io.parity.signer.alerts.Confirm
import io.parity.signer.alerts.ErrorModal
import io.parity.signer.alerts.ShieldAlert
import io.parity.signer.components.Documents
import io.parity.signer.modals.*
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.increment
import io.parity.signer.models.pushButton
import io.parity.signer.screens.*
import io.parity.signer.uniffi.ModalData
import io.parity.signer.uniffi.ScreenData

@ExperimentalAnimationApi
@ExperimentalMaterialApi
@Composable
fun ScreenSelector(signerDataModel: SignerDataModel) {
	val screenData by signerDataModel.screenData.observeAsState()
	val alertState by signerDataModel.alertState.observeAsState()
	val sd = screenData
	Log.w("SSSS", "$sd")
	when (sd) {
		is ScreenData.DeriveKey -> NewAddressScreen(
			sd.f,
			signerDataModel = signerDataModel,
			increment = false
		)
		ScreenData.Documents -> Documents()
		is ScreenData.KeyDetails -> ExportPublicKey(sd.f)
		is ScreenData.KeyDetailsMulti -> KeyDetailsMulti(
			sd.f,
			signerDataModel = signerDataModel
		)
		is ScreenData.Keys -> KeyManager(
			signerDataModel::pushButton,
			signerDataModel::increment,
			sd.f,
			alertState
		)
		is ScreenData.Log -> HistoryScreen(sd.f, signerDataModel = signerDataModel)
		is ScreenData.LogDetails -> LogDetails(sd.f)
		is ScreenData.ManageNetworks -> ManageNetworks(
			sd.f,
			signerDataModel = signerDataModel
		)
		is ScreenData.NNetworkDetails -> NetworkDetails(
			sd.f,
			signerDataModel = signerDataModel
		)
		is ScreenData.NewSeed -> NewSeedScreen(
			sd.f,
			signerDataModel::pushButton,
			signerDataModel = signerDataModel
		)
		is ScreenData.RecoverSeedName -> RecoverSeedName(
			sd.f,
			signerDataModel::pushButton,
			signerDataModel = signerDataModel
		)
		is ScreenData.RecoverSeedPhrase -> RecoverSeedPhrase(
			sd.f,
			signerDataModel::pushButton,
			signerDataModel = signerDataModel
		)
		ScreenData.Scan -> ScanScreen(
			signerDataModel = signerDataModel
		)
		is ScreenData.SeedSelector -> SeedManager(
			sd.f,
			signerDataModel = signerDataModel
		)
		is ScreenData.SelectSeedForBackup -> SelectSeedForBackup(
			sd.f,
			signerDataModel = signerDataModel
		)
		is ScreenData.Settings -> SettingsScreen(
			sd.f,
			signerDataModel = signerDataModel
		)
		is ScreenData.SignSufficientCrypto -> SignSufficientCrypto(
			sd.f,
			signerDataModel = signerDataModel
		)
		is ScreenData.Transaction -> TransactionPreview(
			sd.f,
			signerDataModel::pushButton,
			signerDataModel = signerDataModel
		)
		is ScreenData.VVerifier -> VerifierScreen(sd.f, signerDataModel)
		null -> WaitingScreen()
	}
}

@Composable
fun ModalSelector(
	liveModal: LiveData<ModalData>,
	signerDataModel: SignerDataModel
) {
	val modalState by liveModal.observeAsState()

	val modal = modalState
	Log.w("SIGNER_RUST_LOG", ">>> $modal")

	when (modal) {
		is ModalData.NewSeedMenu -> NewSeedMenu(signerDataModel = signerDataModel)
		is ModalData.SeedMenu -> SeedMenu(
			modal.f,
			signerDataModel = signerDataModel
		)
		is ModalData.NetworkSelector -> NetworkSelector(
			modal.f,
			signerDataModel = signerDataModel
		)
		is ModalData.Backup -> SeedBackup(
			modal.f,
			signerDataModel = signerDataModel
		)
		is ModalData.PasswordConfirm -> PasswordConfirm(
			modal.f,
			signerDataModel = signerDataModel
		)
		is ModalData.SignatureReady -> SignatureReady(
			modal.f,
			signerDataModel = signerDataModel
		)
		is ModalData.EnterPassword -> EnterPassword(
			modal.f,
			signerDataModel = signerDataModel
		)
		is ModalData.LogRight -> LogMenu(modal.f, signerDataModel = signerDataModel)
		is ModalData.NetworkDetailsMenu -> NetworkDetailsMenu(signerDataModel = signerDataModel)
		is ModalData.ManageMetadata -> {
			val screenData = signerDataModel.screenData.value
			if (screenData is ScreenData.NNetworkDetails) {
				ManageMetadata(screenData.f, signerDataModel = signerDataModel)
			}
		}
		is ModalData.SufficientCryptoReady -> SufficientCryptoReady(
			modal.f,
			signerDataModel = signerDataModel
		)
		is ModalData.KeyDetailsAction -> KeyDetailsAction(signerDataModel = signerDataModel)
		is ModalData.TypesInfo -> TypesInfo(
			modal.f,
			signerDataModel = signerDataModel
		)
		is ModalData.NewSeedBackup -> NewSeedBackup(
			modal.f,
			signerDataModel = signerDataModel
		)
		is ModalData.LogComment -> LogComment(signerDataModel = signerDataModel)
		is ModalData.SelectSeed -> {
			val screenData = signerDataModel.screenData.value

			if (screenData is ScreenData.SeedSelector) {
				SelectSeed(screenData.f, signerDataModel = signerDataModel)
			}
		}
		is ModalData.ManageNetworks -> {}
		null -> {}
	}
}

@Composable
fun AlertSelector(alert: SignerAlert, signerDataModel: SignerDataModel) {
	when (alert) {
		SignerAlert.Empty -> {}
		SignerAlert.Error -> ErrorModal(
			error = signerDataModel.alertData.value ?: "unknown error",
			signerDataModel = signerDataModel
		)
		SignerAlert.Shield -> ShieldAlert(signerDataModel)
		SignerAlert.Confirm -> Confirm(signerDataModel = signerDataModel)
	}
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
