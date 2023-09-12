package io.parity.signer.screens.settings.networks.details

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.domain.Callback
import io.parity.signer.domain.FakeNavigator
import io.parity.signer.domain.Navigator
import io.parity.signer.domain.backend.mapError
import io.parity.signer.screens.settings.SettingsNavSubgraph
import io.parity.signer.screens.settings.general.SettingsGeneralNavSubgraph
import io.parity.signer.screens.settings.networks.details.menu.ConfirmRemoveMetadataBottomSheet
import io.parity.signer.screens.settings.networks.details.menu.ConfirmRemoveNetworkBottomSheet
import io.parity.signer.screens.settings.networks.details.menu.NetworkDetailsMenuGeneral
import io.parity.signer.screens.settings.networks.list.NetworkListViewModel
import io.parity.signer.ui.BottomSheetWrapperRoot
import io.parity.signer.ui.mainnavigation.CoreUnlockedNavSubgraph
import io.parity.signer.uniffi.Action
import kotlinx.coroutines.runBlocking


@Composable
fun NetworkDetailsSubgraph(
	networkKey: String,
	navController: NavController,
) {
	//todo dmitry get this model like in
	// ios/PolkadotVault/Backend/NavigationServices/ManageNetworkDetailsService.swift:10
	val vm: NetworkDetailsViewModel = viewModel()

	val model = remember {
		runBlocking {
			vm.getNetworkDetails(networkKey).mapError()!!
			//todo dmitry post error
		}
	}

	val menuController = rememberNavController()
	val savedMetadataVersionAction = remember {
		mutableStateOf("")
	}

	Box(modifier = Modifier.statusBarsPadding()) {
		NetworkDetailsScreen(
			model = model,
			onBack = navController::popBackStack,
			onMenu = { menuController.navigate(NetworkDetailsMenuSubgraph.menu) },
			onRemoveMetadataCallback = { metadataVersion ->
				savedMetadataVersionAction.value = metadataVersion
				menuController.navigate(NetworkDetailsMenuSubgraph.metadataDeleteConfirm)
			},
			onAddNetwork = {
				navController.navigate(CoreUnlockedNavSubgraph.camera)
			},
		)
	}
	val closeAction: Callback = {
		menuController.popBackStack()
	}
	//menus
	NavHost(
		navController = menuController,
		startDestination = NetworkDetailsMenuSubgraph.empty,
	) {
		composable(NetworkDetailsMenuSubgraph.empty) {}
		composable(NetworkDetailsMenuSubgraph.menu) {
			BottomSheetWrapperRoot(onClosedAction = closeAction) {
				NetworkDetailsMenuGeneral(
					onSignNetworkSpecs = {
						navController.navigate(SettingsNavSubgraph.networkSignSufficientCrypto)
					},
					onDeleteClicked = {
						menuController.navigate(NetworkDetailsMenuSubgraph.networkDeleteConfirm) {
							popUpTo(0)
						}
					},
					onCancel = closeAction,
				)
			}
		}
		composable(NetworkDetailsMenuSubgraph.networkDeleteConfirm) {
			BottomSheetWrapperRoot(onClosedAction = closeAction) {
				ConfirmRemoveNetworkBottomSheet(
					onRemoveNetwork = {
						vm.removeNetwork(networkKey)
					},
					onCancel = closeAction,
				)
			}
		}
		composable(NetworkDetailsMenuSubgraph.metadataDeleteConfirm) {
			BottomSheetWrapperRoot(onClosedAction = closeAction) {
				ConfirmRemoveMetadataBottomSheet(
					onRemoveMetadata = {
						//todo dmitry implement
//						FakeNavigator().navigate(
//							Action.MANAGE_METADATA,
//							savedMetadataVersionAction.value
//						)
//						rootNavigator.navigate(Action.REMOVE_METADATA)
						//remove metadata for managed network and stay in network details
						//todo update network details model as well
						closeAction()
					},
					onCancel = closeAction,
				)
			}
		}
	}
}


private object NetworkDetailsMenuSubgraph {
	const val empty = "networkdetails_empty"
	const val menu = "networkdetails_menu"
	const val networkDeleteConfirm = "networkdetails_deleteConfirm"
	const val metadataDeleteConfirm = "networkdetails_metadata_deleteConfirm"
}
