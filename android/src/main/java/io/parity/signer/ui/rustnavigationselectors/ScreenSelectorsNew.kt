package io.parity.signer.ui.rustnavigationselectors

import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import io.parity.signer.bottomsheets.password.EnterPassword
import io.parity.signer.bottomsheets.password.toEnterPasswordModel
import io.parity.signer.components.panels.CameraParentSingleton
import io.parity.signer.domain.Navigator
import io.parity.signer.domain.NetworkState
import io.parity.signer.domain.SharedViewModel
import io.parity.signer.domain.submitErrorState
import io.parity.signer.screens.keysets.create.NewKeysetMenu
import io.parity.signer.ui.BottomSheetWrapperRoot
import io.parity.signer.ui.mainnavigation.CoreUnlockedNavSubgraph
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.ModalData
import io.parity.signer.uniffi.ScreenData

@Composable
fun CombinedScreensSelector(
	screenData: ScreenData,
	sharedViewModel: SharedViewModel
) {
	val rootNavigator = sharedViewModel.navigator

	when (screenData) {
		is ScreenData.SeedSelector -> {
			CoreUnlockedNavSubgraph(
				singleton = sharedViewModel,
			)
		}

		is ScreenData.Keys -> {//keyset details
			submitErrorState("key set details clicked for non existing key details content")
		}

		is ScreenData.KeyDetails -> {
			submitErrorState("key set details clicked for non existing key details content")
		}

		is ScreenData.Log -> {} // moved to settings flow, not part of global state machine now
		is ScreenData.Settings -> {}

		is ScreenData.ManageNetworks -> {}

		is ScreenData.NNetworkDetails -> {}

		is ScreenData.NewSeed -> {
			submitErrorState("nothing, moved to keyset subgraph")
		}

		is ScreenData.RecoverSeedName -> {}

		is ScreenData.RecoverSeedPhrase -> {}

		is ScreenData.Scan -> {	}

		is ScreenData.Transaction -> {
			submitErrorState("Should be unreachable. Local navigation should be used everywhere and this is part of ScanNavSubgraph $screenData")
			CameraParentSingleton.navigateBackFromCamera(rootNavigator)
		}

		is ScreenData.DeriveKey -> {}

		is ScreenData.VVerifier -> {}

		else -> {} //old Selector showing them
	}
}

@Composable
fun BottomSheetSelector(
	modalData: ModalData?,
	networkState: State<NetworkState?>,
	sharedViewModel: SharedViewModel,
	navigator: Navigator,
) {
	SignerNewTheme {
		when (modalData) {
			is ModalData.KeyDetailsAction -> {
				submitErrorState("nothing, moved to KeyDetailsScreenSubgraph")
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


