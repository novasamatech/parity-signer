package io.parity.signer

import android.util.Log
import androidx.compose.animation.ExperimentalAnimationApi
import androidx.compose.material.ExperimentalMaterialApi
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
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
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.ModalData
import io.parity.signer.uniffi.ScreenData

@Composable
fun ScreenSelector(
	screenData: ScreenData,
	alertState: State<ShieldAlert>,
  progress: State<Float?>,
	captured: State<Int?>,
	total: State<Int?>,
	button: (Action, String, String) -> Unit,
	signerDataModel: SignerDataModel
) {
	when (screenData) {
		is ScreenData.DeriveKey -> NewAddressScreen(
			screenData.f,
			signerDataModel = signerDataModel
		)
		ScreenData.Documents -> Documents()
		is ScreenData.KeyDetails -> ExportPublicKey(screenData.f)
		is ScreenData.KeyDetailsMulti -> KeyDetailsMulti(
			screenData.f,
			signerDataModel = signerDataModel
		)
		is ScreenData.Keys -> KeyManager(
			signerDataModel::pushButton,
			signerDataModel::increment,
			screenData.f,
			alertState.value
		)
		is ScreenData.Log -> HistoryScreen(screenData.f, signerDataModel = signerDataModel)
		is ScreenData.LogDetails -> LogDetails(screenData.f)
		is ScreenData.ManageNetworks -> ManageNetworks(
			screenData.f,
			signerDataModel = signerDataModel
		)
		is ScreenData.NNetworkDetails -> NetworkDetails(
			screenData.f,
			signerDataModel = signerDataModel
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
			signerDataModel = signerDataModel
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
fun ModalSelector(modalData: ModalData, signerDataModel: SignerDataModel) {
	when (modalData) {
		is ModalData.Text -> {}
		is ModalData.NewSeedMenu -> NewSeedMenu(signerDataModel = signerDataModel)
		is ModalData.SeedMenu -> SeedMenu(
			modalData.f,
			signerDataModel = signerDataModel
		)
		is ModalData.NetworkSelector -> NetworkSelector(
			modalData.f,
			signerDataModel = signerDataModel
		)
		is ModalData.Backup -> SeedBackup(
			modalData.f,
			signerDataModel = signerDataModel
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
			signerDataModel = signerDataModel
		)
		is ModalData.LogRight -> LogMenu(modalData.f, signerDataModel = signerDataModel)
		is ModalData.NetworkDetailsMenu -> NetworkDetailsMenu(signerDataModel = signerDataModel)
		is ModalData.ManageMetadata -> {
				ManageMetadata(modalData.f, signerDataModel = signerDataModel)
		}
		is ModalData.SufficientCryptoReady -> SufficientCryptoReady(
			modalData.f,
			signerDataModel = signerDataModel
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
		is ModalData.ManageNetworks -> {}
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
		SignerAlert.Confirm -> Confirm(signerDataModel = signerDataModel)
	}
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
