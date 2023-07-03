package io.parity.signer.screens.keysetdetails

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.ui.Modifier
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.domain.Callback
import io.parity.signer.domain.KeySetDetailsModel
import io.parity.signer.domain.Navigator
import io.parity.signer.domain.NetworkState
import io.parity.signer.screens.keysetdetails.filtermenu.NetworkFilterMenu
import io.parity.signer.ui.BottomSheetWrapperRoot

@Composable
fun KeySetDetailsScreenFull(
	fullModel: KeySetDetailsModel,
	navigator: Navigator,
	navController: NavController,
	networkState: State<NetworkState?>, //for shield icon
	onRemoveKeySet: Callback,
) {
	val menuNavController = rememberNavController()

	val keySetViewModel: KeySetDetailsViewModel = viewModel()

	Box(Modifier.statusBarsPadding()) {
		KeySetDetailsScreenView(
			model = fullModel,
			navigator = navigator,
			networkState = networkState,
			onMenu = {
				menuNavController.navigate(KeySetDetailsMenuSubgraph.keys_menu)
			},
			//todo dmitry open KeySetDetailsMenuSubgraph.network_filter
		)
	}

	NavHost(
		navController = menuNavController,
		startDestination = KeySetDetailsMenuSubgraph.empty,
	) {
		val closeAction: Callback = {
			menuNavController.popBackStack()
		}
		composable(KeySetDetailsMenuSubgraph.empty) {}//no menu
		composable(KeySetDetailsMenuSubgraph.keys_menu) {
			BottomSheetWrapperRoot(onClosedAction = closeAction) {
				KeyDetailsMenuGeneral(
					navigator = navigator,
					networkState = networkState,
					onSelectKeysClicked = {
						menuNavController.popBackStack()
						navController.navigate(KeySetDetailsNavSubgraph.multiselect)
					},
					onBackupClicked = {
						menuNavController.popBackStack()
						navController.navigate(KeySetDetailsNavSubgraph.backup)
					},
					onCancel = {
						menuNavController.popBackStack()
					},
					onDeleteClicked = {
						menuNavController.navigate(KeySetDetailsMenuSubgraph.keys_menu_delete_confirm) {
							popUpTo(KeySetDetailsMenuSubgraph.empty)
						}
					}
				)
			}
		}
		composable(KeySetDetailsMenuSubgraph.keys_menu_delete_confirm) {
			BottomSheetWrapperRoot(onClosedAction = closeAction) {
				KeySetDeleteConfirmBottomSheet(
					onCancel = closeAction,
					onRemoveKeySet = onRemoveKeySet,
				)
			}
		}
		composable(KeySetDetailsMenuSubgraph.network_filter) {
			val initialSelection =
				keySetViewModel.filters.collectAsStateWithLifecycle()
			BottomSheetWrapperRoot(onClosedAction = closeAction) {
				NetworkFilterMenu(
					networks = keySetViewModel.getAllNetworks(),
					initialSelection = initialSelection.value,
					onConfirm = {
						keySetViewModel.setFilters(it)
						closeAction()
					},
				)
			}
		}
	}
}


private object KeySetDetailsMenuSubgraph {
	const val empty = "keys_menu_empty"
	const val keys_menu = "keys_menu"
	const val keys_menu_delete_confirm = "keys_menu_delete_confirm"
	const val network_filter = "keys_network_filters"
}

