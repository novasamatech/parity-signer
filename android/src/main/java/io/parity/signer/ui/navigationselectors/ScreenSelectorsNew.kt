package io.parity.signer.ui

import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import io.parity.signer.bottomsheets.KeyDetailsMenuAction
import io.parity.signer.bottomsheets.exportprivatekey.PrivateKeyExportBottomSheet
import io.parity.signer.components.Documents
import io.parity.signer.models.*
import io.parity.signer.screens.*
import io.parity.signer.screens.keysets.KeyManager
import io.parity.signer.screens.keysets.toKeySetsSelectViewModel
import io.parity.signer.ui.navigationselectors.KeySetsNavSubgraph
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.ModalData
import io.parity.signer.uniffi.ScreenData

@Composable
fun CombinedScreensSelector(
	screenData: ScreenData,
	alertState: State<AlertState?>,
	signerDataModel: SignerDataModel
) {
	when (screenData) {
		is ScreenData.SeedSelector -> SignerNewTheme() {
			KeySetsNavSubgraph(
				screenData.f.toKeySetsSelectViewModel(),
				rootNavigator = signerDataModel.navigator,
				alertState = alertState,
			)
		}
		else -> {} //old Selector showing them
	}
}

@Composable
fun BottomSheetSelector(
	modalData: ModalData?,
	localNavAction: LocalNavAction?,
	alertState: State<AlertState?>,
	signerDataModel: SignerDataModel,
	navigator: Navigator,
) {
	SignerNewTheme {

		if (localNavAction != null && localNavAction != LocalNavAction.None) {

			when (localNavAction) {
				is LocalNavAction.ShowExportPrivateKey -> {
					BottomSheetWrapperRoot {
						PrivateKeyExportBottomSheet(
							model = localNavAction.model,
							navigator = localNavAction.navigator
						)
					}
				}
				LocalNavAction.None -> {}
			}

		} else {
			when (modalData) {
				is ModalData.KeyDetailsAction ->
					BottomSheetWrapperRoot(onClosedAction = {
						navigator.backAction()
					} ) {
						KeyDetailsMenuAction(
							navigator = navigator,
							keyDetails = signerDataModel.lastOpenedKeyDetails
						)
					}
				else -> {}
			}
		}
	}
}

