package io.parity.signer.screens.keysetdetails

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.ui.Modifier
import androidx.navigation.NavController
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import androidx.navigation.navArgument
import io.parity.signer.bottomsheets.PublicKeyBottomSheetView
import io.parity.signer.domain.Callback
import io.parity.signer.domain.KeySetDetailsModel
import io.parity.signer.domain.Navigator
import io.parity.signer.domain.NetworkState
import io.parity.signer.domain.submitErrorState
import io.parity.signer.ui.BottomSheetWrapperRoot

@Composable
fun KeySetDetailsScreenFull(
	model: KeySetDetailsModel,
	navigator: Navigator,
	navController: NavController,
	networkState: State<NetworkState?>, //for shield icon
	onRemoveKeySet: Callback,
) {
	val menuNavController = rememberNavController()

	Box(Modifier.statusBarsPadding()) {
		KeySetDetailsScreenView(
			model = model,
			navigator = navigator,
			networkState = networkState,
			onMenu = {
				menuNavController.navigate(KeySetDetailsMenuSubgraph.keys_menu)
			},
			onShowPublicKey = { title: String, key: String ->
				menuNavController.navigate("${KeySetDetailsMenuSubgraph.keys_public_key}/$title/$key")
			},
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
		composable(
			route = "${KeySetDetailsMenuSubgraph.keys_public_key}/{$ARGUMENT_PUBLIC_KEY_TITLE}/{$ARGUMENT_PUBLIC_KEY_VALUE}",
			arguments = listOf(
				navArgument(ARGUMENT_PUBLIC_KEY_TITLE) { type = NavType.StringType },
				navArgument(ARGUMENT_PUBLIC_KEY_VALUE) { type = NavType.StringType }
			)
		) { backStackEntry ->
			val keyName =
				backStackEntry.arguments?.getString(ARGUMENT_PUBLIC_KEY_TITLE) ?: run {
					submitErrorState("mandatory parameter missing for KeySetDetailsMenuSubgraph.keys_public_key")
					""
				}
			val keyValue =
				backStackEntry.arguments?.getString(ARGUMENT_PUBLIC_KEY_VALUE) ?: run {
					submitErrorState("mandatory parameter missing for KeySetDetailsMenuSubgraph.keys_public_key")
					""
				}

			BottomSheetWrapperRoot(onClosedAction = closeAction) {
				PublicKeyBottomSheetView(
					name = keyName,
					key = keyValue,
					onClose = closeAction,
				)
			}
		}
	}
}


private object KeySetDetailsMenuSubgraph {
	const val empty = "keys_menu_empty"
	const val keys_menu = "keys_menu"
	const val keys_menu_delete_confirm = "keys_menu_delete_confirm"
	const val keys_public_key = "keys_public_key"
}

private const val ARGUMENT_PUBLIC_KEY_TITLE = "ARGUMENT_PUBLIC_KEY_TITLE"
private const val ARGUMENT_PUBLIC_KEY_VALUE = "ARGUMENT_PUBLIC_KEY_VALUE"
