package io.parity.signer.ui.navigationselectors

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.imePadding
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Modifier
import io.parity.signer.bottomsheets.EnterPassword
import io.parity.signer.bottomsheets.LogComment
import io.parity.signer.bottomsheets.SignatureReady
import io.parity.signer.models.*
import io.parity.signer.screens.keydetails.KeyDetailsMenuAction
import io.parity.signer.screens.keydetails.KeyDetailsPublicKeyScreen
import io.parity.signer.screens.keydetails.exportprivatekey.PrivateKeyExportBottomSheet
import io.parity.signer.screens.keysets.create.NewKeySetScreen
import io.parity.signer.screens.keysets.create.NewSeedMenu
import io.parity.signer.screens.logs.LogsMenu
import io.parity.signer.screens.logs.LogsScreen
import io.parity.signer.screens.logs.toLogsScreenModel
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
	val seedNames = signerDataModel.seedNames.collectAsState()

	when (localNavAction) {
		LocalNavAction.ShowScan -> {
			ScanNavSubgraph(
				signerDataModel = signerDataModel,
				rootNavigator = rootNavigator
			)
		}
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
			is ScreenData.NewSeed ->
				Box(
					modifier = Modifier
						.statusBarsPadding()
						.imePadding()
				) {
					NewKeySetScreen(
						rootNavigator = rootNavigator,
						seedNames = seedNames.value,
					)
				}
			is ScreenData.Scan, is ScreenData.Transaction ->
				submitErrorState("Should be unreachable. Local navigation should be used everywhere and this is part of ScanNavSubgraph")

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
				is ModalData.NewSeedMenu ->
					//old design
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
				//old design
				is ModalData.SignatureReady -> SignatureReady(
					modalData.f,
					signerDataModel = signerDataModel
				)
				//old design
				is ModalData.EnterPassword -> EnterPassword(
					modalData.f,
				) { action, string -> navigator.navigate(action, string) }
				//old design
				is ModalData.LogComment -> LogComment(signerDataModel = signerDataModel)
				else -> {}
			}
		}
	}
}


