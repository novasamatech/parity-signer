package io.parity.signer.ui.navigationselectors

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.ui.Modifier
import io.parity.signer.bottomsheets.LogComment
import io.parity.signer.models.*
import io.parity.signer.screens.keydetails.KeyDetailsMenuAction
import io.parity.signer.screens.keydetails.exportprivatekey.PrivateKeyExportBottomSheet
import io.parity.signer.screens.keydetails.KeyDetailsPublicKeyScreen
import io.parity.signer.screens.keysets.NewSeedMenu
import io.parity.signer.screens.logs.LogsMenu
import io.parity.signer.screens.logs.LogsScreen
import io.parity.signer.screens.logs.toLogsScreenModel
import io.parity.signer.screens.scan.ScanScreen
import io.parity.signer.ui.BottomSheetWrapperRoot
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.uniffi.ModalData
import io.parity.signer.uniffi.ScreenData

@Composable
fun CombinedScreensSelector(
	screenData: ScreenData,
	localNavAction: LocalNavAction?,
	alertState: State<AlertState?>,
	signerDataModel: SignerDataModel
) {
	val rootNavigator = signerDataModel.navigator

	when (localNavAction) {
		LocalNavAction.ShowScan ->
			ScanScreen(
				onClose = { rootNavigator.backAction() }
			)
		else -> when (screenData) {
			is ScreenData.SeedSelector -> {
				KeySetsNavSubgraph(
					screenData.f.toKeySetsSelectModel(),
					rootNavigator = rootNavigator,
					alertState = alertState,
				)
			}
			is ScreenData.Keys ->
				KeySetDetailsNavSubgraph(
					model = screenData.f.toKeySetDetailsModel(),
					rootNavigator = rootNavigator,
					alertState = alertState,
					singleton = signerDataModel,
				)
			is ScreenData.KeyDetails ->
				Box(modifier = Modifier.statusBarsPadding()) {
					KeyDetailsPublicKeyScreen(
						model = screenData.f.toKeyDetailsModel(),
						rootNavigator = rootNavigator,
					)
				}
			is ScreenData.Log ->
				Box(Modifier.statusBarsPadding()) {
					LogsScreen(
						model = screenData.f.toLogsScreenModel(),
						navigator = rootNavigator,
					)
				}
			is ScreenData.Scan -> throw RuntimeException("how did you get there? Local navigation should be used everywhere")

			else -> {} //old Selector showing them
		}
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
				else -> {}
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
				//todo change design to new
				is ModalData.NewSeedMenu ->
					BottomSheetWrapperRoot(onClosedAction = {
						navigator.backAction()
					}) {
						NewSeedMenu(
							alertState = alertState,
							navigator = signerDataModel.navigator,
						)
					}
				is ModalData.LogRight ->
					BottomSheetWrapperRoot(onClosedAction = {
						navigator.backAction()
					}) {
						LogsMenu(
							navigator = signerDataModel.navigator,
						)
					}
				//todo update design
				is ModalData.LogComment -> LogComment(signerDataModel = signerDataModel)
				else -> {}
			}
		}
	}
}


