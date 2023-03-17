package io.parity.signer.screens.networks.details

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.domain.Callback
import io.parity.signer.domain.FakeNavigator
import io.parity.signer.domain.Navigator
import io.parity.signer.screens.networks.details.menu.ConfirmRemoveNetworkBottomSheet
import io.parity.signer.screens.networks.details.menu.NetworkDetailsMenuGeneral
import io.parity.signer.ui.BottomSheetWrapperRoot
import io.parity.signer.uniffi.Action


@Composable
fun NetworkDetailsSubgraph(
	model: NetworkDetailsModel,
	rootNavigator: Navigator,
) {
	val menuController = rememberNavController()

	Box(modifier = Modifier.statusBarsPadding()) {
		NetworkDetailsScreen(
			model = model,
			rootNavigator = rootNavigator,
			onMenu = { menuController.navigate(NetworkDetailsMenuSubgraph.menu) },
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
						FakeNavigator().navigate(Action.RIGHT_BUTTON_ACTION)
						rootNavigator.navigate(Action.SIGN_NETWORK_SPECS)
					},
					onDeleteClicked = {
						menuController.navigate(NetworkDetailsMenuSubgraph.deleteConfirm) {
							popUpTo(0)
						}
					},
					onCancel = closeAction,
				)
			}
		}
		composable(NetworkDetailsMenuSubgraph.deleteConfirm) {
			BottomSheetWrapperRoot(onClosedAction = closeAction) {
				ConfirmRemoveNetworkBottomSheet(
					onRemoveKey = {
						FakeNavigator().navigate(Action.RIGHT_BUTTON_ACTION)
						rootNavigator.navigate(Action.REMOVE_NETWORK)
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
	const val deleteConfirm = "networkdetails_deleteConfirm"
}
