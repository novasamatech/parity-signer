package io.parity.signer.ui.rustnavigationselectors

import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.runtime.collectAsState
import io.parity.signer.alerts.Confirm
import io.parity.signer.alerts.ErrorModal
import io.parity.signer.components.exposesecurity.ShieldAlert
import io.parity.signer.bottomsheets.*
import io.parity.signer.components.Documents
import io.parity.signer.domain.*
import io.parity.signer.domain.storage.addSeed
import io.parity.signer.domain.storage.signSufficientCrypto
import io.parity.signer.screens.*
import io.parity.signer.screens.logs.logdetails.LogDetails
import io.parity.signer.screens.networks.NetworkDetails
import io.parity.signer.screens.settings.VerifierScreen
import io.parity.signer.ui.theme.SignerOldTheme
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.AlertData
import io.parity.signer.uniffi.ModalData
import io.parity.signer.uniffi.ScreenData

@Composable
fun ScreenSelector(
	screenData: ScreenData,
	alertState: State<AlertState?>,
	navigate: (Action, String, String) -> Unit,
	signerDataModel: SignerDataModel
) {
	val button2: (Action, String) -> Unit =
		{ action, details -> navigate(action, details, "") }
	val seedNames =
		signerDataModel.seedStorage.lastKnownSeedNames.collectAsState()

	when (screenData) {
		is ScreenData.DeriveKey -> {} // migrated
		ScreenData.Documents -> Documents()
		is ScreenData.KeyDetails -> {}//migrated
		is ScreenData.KeyDetailsMulti -> {
			//migrated, now part of KeySetDetails subgraph and old data used
			submitErrorState(
				"unreacheble state reached - root navigator should never " +
					"get to Key Details Multi $screenData"
			)
		}
		is ScreenData.Keys -> {} //migrated to new selector
		is ScreenData.Log -> {} //migrated to new selector
		is ScreenData.LogDetails -> LogDetails(screenData.f)
		is ScreenData.ManageNetworks -> ManageNetworks(
			screenData.f,
			button2,
		)
		is ScreenData.NNetworkDetails -> NetworkDetails(
			screenData.f,
			button2
		)
		is ScreenData.NewSeed -> {} // new selector
		is ScreenData.RecoverSeedName -> RecoverSeedName(
			screenData.f,
			signerDataModel::navigate,
			seedNames.value
		)
		is ScreenData.RecoverSeedPhrase -> RecoverSeedPhrase(
			recoverSeedPhrase = screenData.f,
			button = signerDataModel::navigate,
			addSeed = signerDataModel::addSeed
		)
		ScreenData.Scan -> {} //in new selector
		is ScreenData.Transaction -> {} //in new selector
		is ScreenData.SeedSelector -> {} //shown in new selector
		is ScreenData.SelectSeedForBackup -> SelectSeedForBackup(
			screenData.f,
			button2
		)
		is ScreenData.Settings -> {} //new selector
		is ScreenData.SignSufficientCrypto -> SignSufficientCrypto(
			screenData.f,
			signerDataModel::signSufficientCrypto
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
	localNavAction: LocalNavAction?,
	alertState: State<AlertState?>,
	navigate: (Action, String, String) -> Unit,
	signerDataModel: SignerDataModel
) {
	val button2: (Action, String) -> Unit =
		{ action, details -> navigate(action, details, "") }
	if (localNavAction != null && localNavAction != LocalNavAction.None) {
		when (localNavAction) {
			is LocalNavAction.ShowExportPrivateKey -> {} //show in new selector
			else -> {}
		}
	} else {
		when (modalData) {
			is ModalData.NewSeedMenu -> {} //new bottom sheet
			is ModalData.SeedMenu -> {} //migrated
			is ModalData.NetworkSelector -> NetworkSelector(
				modalData.f,
				button2,
			)
			is ModalData.Backup -> {} //new screen is part of key details subgraph
			is ModalData.PasswordConfirm -> {
				//this is part of Derivation flow and should never called here
				submitErrorState(
					"unreacheble state reached - root navigator should never " +
						"get to confirm password as it's part derivation details and never " +
						"called now $modalData"
				)
			}
			is ModalData.SignatureReady -> {} //in new selector
			is ModalData.EnterPassword -> {} //in new selector
			is ModalData.LogRight -> {} //migrated to bottom sheet
			is ModalData.NetworkDetailsMenu -> NetworkDetailsMenu(
				signerDataModel = signerDataModel
			)
			is ModalData.ManageMetadata -> {
				ManageMetadata(modalData.f, signerDataModel = signerDataModel)
			}
			is ModalData.SufficientCryptoReady -> SufficientCryptoReady(
				modalData.f,
			)
			is ModalData.KeyDetailsAction -> {} //migrated to bottom sheet
			is ModalData.TypesInfo -> TypesInfo(
				modalData.f,
				signerDataModel = signerDataModel
			)
			is ModalData.NewSeedBackup -> {}//moved to new selector
			is ModalData.LogComment -> {} //moved to new sheet
			is ModalData.SelectSeed -> {
				submitErrorState("This is part of refactored screen and not shown separately")
				SelectSeed(modalData.f, signerDataModel = signerDataModel)
			}
			null -> {}
		}
	}
}

@Composable
fun AlertSelector(
	alert: AlertData?,
	alertState: State<AlertState?>,
	navigate: (Action, String, String) -> Unit,
	acknowledgeWarning: Callback
) {
	val button1: (Action) -> Unit = { action -> navigate(action, "", "") }
	SignerOldTheme() {
		when (alert) {
			AlertData.Confirm -> Confirm(button = button1)
			is AlertData.ErrorData -> ErrorModal(
				error = alert.f,
				button = button1
			)
			is AlertData.Shield -> ShieldAlert(
				// alert.f,
				alertState = alertState,
				navigateBack = { button1(Action.GO_BACK) },
				acknowledgeWarning = acknowledgeWarning
			)
			null -> {}
		}
	}
}
