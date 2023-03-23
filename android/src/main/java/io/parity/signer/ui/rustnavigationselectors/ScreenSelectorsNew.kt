package io.parity.signer.ui.rustnavigationselectors

import android.util.Log
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.imePadding
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Modifier
import io.parity.signer.bottomsheets.LogComment
import io.parity.signer.bottomsheets.password.EnterPassword
import io.parity.signer.bottomsheets.password.toEnterPasswordModel
import io.parity.signer.components.panels.CameraParentSingleton
import io.parity.signer.domain.*
import io.parity.signer.domain.storage.addSeed
import io.parity.signer.screens.createderivation.DerivationCreateSubgraph
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
import io.parity.signer.screens.settings.SettingsScreenSubgraph
import io.parity.signer.screens.settings.networks.details.NetworkDetailsSubgraph
import io.parity.signer.screens.settings.networks.details.toNetworkDetailsModel
import io.parity.signer.screens.settings.networks.list.NetworksList
import io.parity.signer.screens.settings.networks.list.toNetworksListModel
import io.parity.signer.ui.BottomSheetWrapperRoot
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.uniffi.*

@Composable
fun CombinedScreensSelector(
	screenData: ScreenData,
	localNavAction: LocalNavAction?,
	networkState: State<NetworkState?>,
	sharedViewModel: SharedViewModel
) {
	val rootNavigator = sharedViewModel.navigator
	val seedNames =
		sharedViewModel.seedStorage.lastKnownSeedNames.collectAsState()

	when (screenData) {
		is ScreenData.SeedSelector -> {
			KeySetsNavSubgraph(
				screenData.f.toKeySetsSelectModel(),
				rootNavigator = rootNavigator,
				networkState = networkState,
			)
		}
		is ScreenData.Keys -> {
			val keys = try {
				keysBySeedName(screenData.f)
			} catch (e: ErrorDisplayed) {
				rootNavigator.backAction()
				submitErrorState("unexpected error in keysBySeedName $e")
				null
			}
			keys?.let {
				KeySetDetailsNavSubgraph(
					model = keys.toKeySetDetailsModel(),
					rootNavigator = rootNavigator,
					networkState = networkState,
					singleton = sharedViewModel,
				)
			}
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
			SettingsScreenSubgraph(
				rootNavigator = rootNavigator,
				isStrongBoxProtected = sharedViewModel.seedStorage.isStrongBoxProtected,
				appVersion = sharedViewModel.getAppVersion(),
				wipeToFactory = sharedViewModel::wipeToFactory,
				networkState = networkState
			)
		is ScreenData.ManageNetworks ->
			Box(modifier = Modifier.statusBarsPadding()) {
				NetworksList(
					model = screenData.f.toNetworksListModel(),
					rootNavigator = rootNavigator
				)
			}
		is ScreenData.NNetworkDetails ->
			NetworkDetailsSubgraph(
				screenData.f.toNetworkDetailsModel(),
				rootNavigator
			)
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
		is ScreenData.Scan -> {
			ScanNavSubgraph(
				rootNavigator = rootNavigator
			)
		}
		is ScreenData.Transaction -> {
			Log.e(
				"Selector",
				"Should be unreachable. Local navigation should be used everywhere and this is part of ScanNavSubgraph $screenData"
			)
			CameraParentSingleton.navigateBackFromCamera(rootNavigator)
		}
		is ScreenData.DeriveKey -> {
			Box(
				modifier = Modifier
					.background(MaterialTheme.colors.background)
			) {
				DerivationCreateSubgraph(
					rootNavigator, screenData.f.seedName,
				)
			}
		}
		else -> {} //old Selector showing them
	}
}

@Composable
fun BottomSheetSelector(
	modalData: ModalData?,
	localNavAction: LocalNavAction?,
	networkState: State<NetworkState?>,
	sharedViewModel: SharedViewModel,
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
							keyDetails = sharedViewModel.lastOpenedKeyDetails
						)
					}
				is ModalData.NewSeedMenu ->
					//old design
					BottomSheetWrapperRoot(onClosedAction = {
						navigator.backAction()
					}) {
						NewSeedMenu(
							networkState = networkState,
							navigator = sharedViewModel.navigator,
						)
					}
				is ModalData.NewSeedBackup -> {
					NewKeySetBackupScreenFull(
						model = modalData.f.toNewSeedBackupModel(),
						onBack = { navigator.backAction() },
						onCreateKeySet = sharedViewModel::addSeed
					)
				}
				is ModalData.LogRight ->
					BottomSheetWrapperRoot(onClosedAction = {
						navigator.backAction()
					}) {
						LogsMenu(
							navigator = sharedViewModel.navigator,
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
				is ModalData.LogComment -> LogComment(sharedViewModel = sharedViewModel)
				else -> {}
			}
		}
	}
}


