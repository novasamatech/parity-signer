package io.parity.signer.ui.navigationselectors

import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import io.parity.signer.screens.keydetails.exportprivatekey.PrivateKeyExportBottomSheet
import io.parity.signer.models.*
import io.parity.signer.screens.keydetails.KeyDetailsMenuAction
import io.parity.signer.screens.keysets.NewSeedMenu
import io.parity.signer.ui.BottomSheetWrapperRoot
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.uniffi.ModalData
import io.parity.signer.uniffi.ScreenData

@Composable
fun CombinedScreensSelector(
	screenData: ScreenData,
	alertState: State<AlertState?>,
	signerDataModel: SignerDataModel
) {
	val rootNavigator = signerDataModel.navigator

	when (screenData) {
		is ScreenData.SeedSelector -> {
			SignerNewTheme() {
				KeySetsNavSubgraph(
					screenData.f.toKeySetsSelectModel(),
					rootNavigator = rootNavigator,
					alertState = alertState,
				)
			}
		}
		is ScreenData.Keys ->
			KeySetDetailsNavSubgraph(
				model = screenData.f.toKeySetDetailsModel(),
				rootNavigator = rootNavigator,
				alertState = alertState,
				sigleton = signerDataModel,
			)
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
					}) {
						KeyDetailsMenuAction(
							navigator = navigator,
							keyDetails = signerDataModel.lastOpenedKeyDetails
						)
					}
				is ModalData.NewSeedMenu ->
					BottomSheetWrapperRoot(onClosedAction = {
						navigator.backAction()
					}) {
						NewSeedMenu(
							alertState = alertState,
							navigator = signerDataModel.navigator,
						)
					}
				else -> {}
			}
		}
	}
}

