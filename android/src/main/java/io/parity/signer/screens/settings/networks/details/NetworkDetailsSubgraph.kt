package io.parity.signer.screens.settings.networks.details

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.domain.Callback
import io.parity.signer.domain.FakeNavigator
import io.parity.signer.domain.Navigator
import io.parity.signer.screens.settings.networks.details.menu.ConfirmRemoveMetadataBottomSheet
import io.parity.signer.screens.settings.networks.details.menu.ConfirmRemoveNetworkBottomSheet
import io.parity.signer.screens.settings.networks.details.menu.NetworkDetailsMenuGeneral
import io.parity.signer.ui.BottomSheetWrapperRoot
import io.parity.signer.uniffi.Action


@Composable
fun NetworkDetailsSubgraph(
    model: NetworkDetailsModel,
    rootNavigator: Navigator,
) {
	//todo dmitry get this model like in
	// ios/PolkadotVault/Backend/NavigationServices/ManageNetworkDetailsService.swift:10
	val menuController = rememberNavController()
	val savedMetadataVersionAction = remember {
		mutableStateOf("")
	}

	Box(modifier = Modifier.statusBarsPadding()) {
		NetworkDetailsScreen(
			model = model,
			rootNavigator = rootNavigator,
			onMenu = { menuController.navigate(NetworkDetailsMenuSubgraph.menu) },
			onRemoveMetadataCallback = { metadataVersion ->
				savedMetadataVersionAction.value = metadataVersion
				menuController.navigate(NetworkDetailsMenuSubgraph.metadataDeleteConfirm)
			},
			onAddNetwork = {
				rootNavigator.backAction()
				rootNavigator.backAction()
				rootNavigator.navigate(Action.NAVBAR_SCAN)
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
						closeAction()
						FakeNavigator().navigate(Action.RIGHT_BUTTON_ACTION)
						rootNavigator.navigate(Action.SIGN_NETWORK_SPECS)
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
						FakeNavigator().navigate(Action.RIGHT_BUTTON_ACTION)
						rootNavigator.navigate(Action.REMOVE_NETWORK)
						closeAction()
					},
					onCancel = closeAction,
				)
			}
		}
		composable(NetworkDetailsMenuSubgraph.metadataDeleteConfirm) {
			BottomSheetWrapperRoot(onClosedAction = closeAction) {
				ConfirmRemoveMetadataBottomSheet(
					onRemoveMetadata = {
						FakeNavigator().navigate(
							Action.MANAGE_METADATA,
							savedMetadataVersionAction.value
						)
						rootNavigator.navigate(Action.REMOVE_METADATA)
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
