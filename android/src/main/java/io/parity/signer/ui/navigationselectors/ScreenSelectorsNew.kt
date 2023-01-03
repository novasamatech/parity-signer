package io.parity.signer.ui.navigationselectors

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.imePadding
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Modifier
import io.parity.signer.bottomsheets.LogComment
import io.parity.signer.bottomsheets.password.EnterPassword
import io.parity.signer.bottomsheets.password.toEnterPasswordModel
import io.parity.signer.models.*
import io.parity.signer.models.storage.addSeed
import io.parity.signer.screens.keydetails.KeyDetailsMenuAction
import io.parity.signer.screens.keydetails.KeyDetailsPublicKeyScreen
import io.parity.signer.screens.keydetails.exportprivatekey.PrivateKeyExportBottomSheet
import io.parity.signer.screens.keysetdetails.KeySetDetailsNavSubgraph
import io.parity.signer.screens.keysets.KeySetsNavSubgraph
import io.parity.signer.screens.keysets.create.NewKeySetBackupScreenFull
import io.parity.signer.screens.keysets.create.NewKeySetNameScreen
import io.parity.signer.screens.keysets.create.NewSeedMenu
import io.parity.signer.screens.keysets.create.toNewSeedBackupModel
import io.parity.signer.screens.logs.LogsMenu
import io.parity.signer.screens.logs.LogsScreen
import io.parity.signer.screens.logs.toLogsScreenModel
import io.parity.signer.screens.scan.ScanNavSubgraph
import io.parity.signer.screens.settings.SettingsScreen
import io.parity.signer.ui.BottomSheetWrapperRoot
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.ModalData
import io.parity.signer.uniffi.ScreenData
import io.parity.signer.uniffi.keysBySeedName

@Composable
fun CombinedScreensSelector(
	screenData: ScreenData,
	localNavAction: LocalNavAction?,
	alertState: State<AlertState?>,
	signerDataModel: SignerDataModel
) {
	val rootNavigator = signerDataModel.navigator
	val seedNames =
		signerDataModel.seedStorage.lastKnownSeedNames.collectAsState()

	when (localNavAction) {
		LocalNavAction.ShowScan -> {
			ScanNavSubgraph(
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
			is ScreenData.Keys -> {
				val keys = keysBySeedName(signerDataModel.dbName, screenData.f)
				KeySetDetailsNavSubgraph(
					model = keys.toKeySetDetailsModel(),
					rootNavigator = rootNavigator,
					alertState = alertState,
					singleton = signerDataModel,
				)
			}
			is ScreenData.KeyDetails ->
				Box(modifier = Modifier.statusBarsPadding()) {
					screenData.f?.toKeyDetailsModel()?.let { model ->
						KeyDetailsPublicKeyScreen(
							model = model,
							rootNavigator = rootNavigator,
						)
					}
						?: run {
							submitErrorState("key details clicked for non existing key details content")
							rootNavigator.backAction()
						}
				}
			is ScreenData.Log ->
				Box(Modifier.statusBarsPadding()) {
					LogsScreen(
						model = screenData.f.toLogsScreenModel(),
						navigator = rootNavigator,
					)
				}
			is ScreenData.Settings ->
				Box(modifier = Modifier.statusBarsPadding()) {
					SettingsScreen(
						rootNavigator = rootNavigator,
						isStrongBoxProtected = signerDataModel.seedStorage.isStrongBoxProtected,
						appVersion = signerDataModel.getAppVersion(),
						wipeToFactory = signerDataModel::wipeToFactory,
						alertState = alertState
					)
				}
			is ScreenData.NewSeed ->
				Box(
					modifier = Modifier
                        .statusBarsPadding()
                        .imePadding()
				) {
					NewKeySetNameScreen(
						rootNavigator = rootNavigator,
						seedNames = seedNames.value,
					)
				}
			is ScreenData.Scan, is ScreenData.Transaction ->
				submitErrorState("Should be unreachable. Local navigation should be used everywhere and this is part of ScanNavSubgraph $screenData")

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
				is ModalData.NewSeedBackup -> {
					NewKeySetBackupScreenFull(
						model = modalData.f.toNewSeedBackupModel(),
						onBack = { navigator.backAction() },
						onCreateKeySet = signerDataModel::addSeed
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
				is ModalData.EnterPassword ->
					BottomSheetWrapperRoot(onClosedAction = {
						navigator.backAction()
					}) {
						EnterPassword(
							modalData.f.toEnterPasswordModel(),
							proceed = { password ->
								navigator.navigate(
									Action.GO_FORWARD,
									password
								)
							},
							onClose = { navigator.backAction() },
						)
					}
				is ModalData.SignatureReady -> {}//part of camera flow now
				//old design
				is ModalData.LogComment -> LogComment(signerDataModel = signerDataModel)
				else -> {}
			}
		}
	}
}


