package io.parity.signer.ui.rustnavigationselectors

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.imePadding
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import io.parity.signer.bottomsheets.password.EnterPassword
import io.parity.signer.bottomsheets.password.toEnterPasswordModel
import io.parity.signer.components.panels.CameraParentSingleton
import io.parity.signer.domain.LocalNavAction
import io.parity.signer.domain.Navigator
import io.parity.signer.domain.NetworkState
import io.parity.signer.domain.SharedViewModel
import io.parity.signer.domain.submitErrorState
import io.parity.signer.domain.toKeyDetailsModel
import io.parity.signer.domain.toKeySetDetailsModel
import io.parity.signer.domain.toKeySetsSelectModel
import io.parity.signer.domain.toVerifierDetailsModels
import io.parity.signer.screens.createderivation.DerivationCreateSubgraph
import io.parity.signer.screens.keydetails.KeyDetailsMenuAction
import io.parity.signer.screens.keydetails.KeyDetailsPublicKeyScreen
import io.parity.signer.screens.keydetails.exportprivatekey.PrivateKeyExportBottomSheet
import io.parity.signer.screens.keysetdetails.KeySetDetailsNavSubgraph
import io.parity.signer.screens.keysets.KeySetsNavSubgraph
import io.parity.signer.screens.keysets.create.NewKeysetMenu
import io.parity.signer.screens.keysets.restore.KeysetRecoverNameScreen
import io.parity.signer.screens.keysets.restore.NewKeysetRecoverSecondStepSubgraph
import io.parity.signer.screens.keysets.restore.toKeysetRecoverModel
import io.parity.signer.screens.scan.ScanNavSubgraph
import io.parity.signer.screens.settings.SettingsScreenSubgraph
import io.parity.signer.screens.settings.networks.details.NetworkDetailsSubgraph
import io.parity.signer.screens.settings.networks.details.toNetworkDetailsModel
import io.parity.signer.screens.settings.networks.list.NetworksListSubgraph
import io.parity.signer.screens.settings.networks.list.toNetworksListModel
import io.parity.signer.screens.settings.verifiercert.VerifierScreenFull
import io.parity.signer.ui.BottomSheetWrapperRoot
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.ErrorDisplayed
import io.parity.signer.uniffi.ModalData
import io.parity.signer.uniffi.ScreenData
import io.parity.signer.uniffi.keysBySeedName

@Composable
fun CombinedScreensSelector(
	screenData: ScreenData,
	localNavAction: LocalNavAction?,
	networkState: State<NetworkState?>,
	sharedViewModel: SharedViewModel
) {
	val rootNavigator = sharedViewModel.navigator
	val seedNames =
		sharedViewModel.seedStorage.lastKnownSeedNames.collectAsStateWithLifecycle()

	when (screenData) {
		is ScreenData.SeedSelector -> {
			KeySetsNavSubgraph(
				screenData.f.toKeySetsSelectModel(),
				rootNavigator = rootNavigator,
				networkState = networkState,
			)
		}

		is ScreenData.Keys -> {
				KeySetDetailsNavSubgraph(
					seedName = screenData.f,
					rootNavigator = rootNavigator,
					networkState = networkState,
					singleton = sharedViewModel,
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

		is ScreenData.Log -> {} // moved to settings flow, not part of global state machine now
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
				NetworksListSubgraph(
					model = screenData.f.toNetworksListModel(),
					rootNavigator = rootNavigator
				)
			}

		is ScreenData.NNetworkDetails ->
			NetworkDetailsSubgraph(
				screenData.f.toNetworkDetailsModel(),
				rootNavigator,
			)

		is ScreenData.NewSeed -> {
			submitErrorState("nothing, moved to keyset subgraph")
		}

		is ScreenData.RecoverSeedName -> {
			Box(
				modifier = Modifier
					.statusBarsPadding()
					.imePadding()
			) {
				KeysetRecoverNameScreen(
					rootNavigator = rootNavigator,
					seedNames = seedNames.value,
				)
			}
		}

		is ScreenData.RecoverSeedPhrase ->
			Box(
				modifier = Modifier
					.statusBarsPadding()
					.imePadding()
			) {
				NewKeysetRecoverSecondStepSubgraph(
					initialRecoverSeedPhrase = screenData.f.toKeysetRecoverModel(),
					rootNavigator = rootNavigator,
				)
			}

		is ScreenData.Scan -> {
			ScanNavSubgraph(
				rootNavigator = rootNavigator
			)
		}

		is ScreenData.Transaction -> {
			submitErrorState("Should be unreachable. Local navigation should be used everywhere and this is part of ScanNavSubgraph $screenData")
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

		is ScreenData.VVerifier -> VerifierScreenFull(
			screenData.f.toVerifierDetailsModels(),
			sharedViewModel::wipeToJailbreak,
			rootNavigator,
		)

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
					BottomSheetWrapperRoot(onClosedAction = {
//						don't do action here because timer's finally will do back navigation
//						navigator.backAction()
					}) {
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
						NewKeysetMenu(
							networkState = networkState,
							navigator = sharedViewModel.navigator,
						)
					}

				is ModalData.NewSeedBackup -> {
					// it is second step in
					submitErrorState("nothing, moved to keyset subgraph")
				}

				is ModalData.LogRight -> {} // moved to settings flow, not part of global state machine now
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

				is ModalData.SignatureReady -> {} //part of camera flow now
				//old design
				is ModalData.LogComment -> {} //moved to logs subgraph, part of settigns now
				else -> {}
			}
		}
	}
}


