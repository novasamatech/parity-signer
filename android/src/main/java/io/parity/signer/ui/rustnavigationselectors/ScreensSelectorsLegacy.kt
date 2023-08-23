package io.parity.signer.ui.rustnavigationselectors

import androidx.compose.runtime.Composable
import io.parity.signer.alerts.Confirm
import io.parity.signer.alerts.ErrorModal
import io.parity.signer.bottomsheets.SelectSeed
import io.parity.signer.bottomsheets.SufficientCryptoReady
import io.parity.signer.components.exposesecurity.ExposedAlert
import io.parity.signer.domain.Callback
import io.parity.signer.domain.LocalNavAction
import io.parity.signer.domain.Navigator
import io.parity.signer.domain.SharedViewModel
import io.parity.signer.domain.storage.signSufficientCrypto
import io.parity.signer.domain.submitErrorState
import io.parity.signer.screens.SelectSeedForBackup
import io.parity.signer.screens.SignSufficientCrypto
import io.parity.signer.ui.theme.SignerOldTheme
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.AlertData
import io.parity.signer.uniffi.ModalData
import io.parity.signer.uniffi.ScreenData

@Composable
fun ScreenSelector(
	screenData: ScreenData,
	navigator: Navigator,
	sharedViewModel: SharedViewModel
) {

	when (screenData) {
		is ScreenData.SelectSeedForBackup -> SelectSeedForBackup(
			screenData.f,
			navigator,
		)

		is ScreenData.SignSufficientCrypto -> SignSufficientCrypto(
			screenData.f,
			sharedViewModel::signSufficientCrypto
		)

		is ScreenData.KeyDetailsMulti -> {
			//migrated, now part of KeySetDetails subgraph and old data used
			submitErrorState(
				"unreacheble state reached - root navigator should never " +
					"get to Key Details Multi $screenData"
			)
		}

		ScreenData.Documents -> {
			submitErrorState(
				"This screen was called from settings but we don't call it anymore.\n" +
					"While I cannot guarantee that rust won't make this state for whatever reason."
			)
		}

		is ScreenData.DeriveKey -> {} // migrated
		is ScreenData.KeyDetails -> {}//migrated
		is ScreenData.Keys -> {} //migrated to new selector
		is ScreenData.Log -> {} //migrated to new selector
		is ScreenData.LogDetails -> {} // moved to settings flow, not part of global state machine now
		is ScreenData.ManageNetworks -> {} //migrated to new selector
		is ScreenData.NNetworkDetails -> {} // migrated to new selector
		is ScreenData.NewSeed -> {} // new selector
		is ScreenData.RecoverSeedName -> {}//new selector
		is ScreenData.RecoverSeedPhrase -> {}//new selector
		ScreenData.Scan -> {} //in new selector
		is ScreenData.Transaction -> {} //in new selector
		is ScreenData.SeedSelector -> {} //shown in new selector
		is ScreenData.Settings -> {} //new selector
		is ScreenData.VVerifier -> {} //new selector
	}
}

@Composable
fun ModalSelector(
	modalData: ModalData?,
	localNavAction: LocalNavAction?,
	sharedViewModel: SharedViewModel
) {
	if (localNavAction != null && localNavAction != LocalNavAction.None) {
		when (localNavAction) {
			is LocalNavAction.ShowExportPrivateKey -> {} //show in new selector
			else -> {}
		}
	} else {
		when (modalData) {
			is ModalData.NewSeedMenu -> {} //new bottom sheet
			is ModalData.SeedMenu -> {} //migrated
			is ModalData.NetworkSelector -> {
				//seed details have no selector anymore but keys are grouped by network
				submitErrorState(
					"unreacheble state reached - network selector action is removed from " +
						"key set details and never called now $modalData"
				)
			}

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
			is ModalData.NetworkDetailsMenu -> {} // migrated to network details screen
			is ModalData.ManageMetadata -> {} // those actions now right in network details screen
			is ModalData.SufficientCryptoReady -> SufficientCryptoReady(
				modalData.f,
			)

			is ModalData.KeyDetailsAction -> {} //migrated to bottom sheet
			is ModalData.TypesInfo -> {} // this functionality removed after redesign
			is ModalData.NewSeedBackup -> {}//moved to new selector
			is ModalData.LogComment -> {} //moved to new sheet
			is ModalData.SelectSeed -> {
				submitErrorState("This is part of refactored screen and not shown separately")
				SelectSeed(modalData.f, sharedViewModel = sharedViewModel)
			}

			null -> {}
		}
	}
}

@Composable
fun AlertSelector(
	alert: AlertData?,
	navigator: Navigator,
	acknowledgeWarning: Callback
) {
	SignerOldTheme() {
		when (alert) {
			AlertData.Confirm -> Confirm(button = { navigator.navigate(it) })
			is AlertData.ErrorData -> ErrorModal(
				error = alert.f,
				button = { action -> navigator.navigate(action) }
			)

			is AlertData.Shield -> ExposedAlert(
				navigateBack = { navigator.navigate(Action.GO_BACK) },
			)

			null -> {}
		}
	}
}
