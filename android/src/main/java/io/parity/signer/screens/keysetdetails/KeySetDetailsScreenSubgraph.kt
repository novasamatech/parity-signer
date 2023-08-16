package io.parity.signer.screens.keysetdetails

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
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
import io.parity.signer.domain.getSeedPhraseForBackup
import io.parity.signer.domain.submitErrorState
import io.parity.signer.screens.keysetdetails.backup.KeySetBackupFullOverlayBottomSheet
import io.parity.signer.screens.keysetdetails.backup.toSeedBackupModel
import io.parity.signer.screens.keysetdetails.export.KeySetDetailsExportResultBottomSheet
import io.parity.signer.screens.keysetdetails.export.KeySetDetailsMultiselectBottomSheet
import io.parity.signer.screens.keysetdetails.filtermenu.NetworkFilterMenu
import io.parity.signer.ui.BottomSheetWrapperRoot

@Composable
fun KeySetDetailsScreenSubgraph(
	fullModel: KeySetDetailsModel,
	navigator: Navigator,
	navController: NavController,
	networkState: State<NetworkState?>, //for shield icon
	onRemoveKeySet: Callback,
) {
	val menuNavController = rememberNavController()

	val keySetViewModel: KeySetDetailsViewModel = viewModel()
	val filteredModel =
		keySetViewModel.makeFilteredFlow(fullModel).collectAsStateWithLifecycle()

	Box(Modifier.statusBarsPadding()) {
		KeySetDetailsScreenView(
			model = filteredModel.value,
			navigator = navigator,
			networkState = networkState,
			fullModelWasEmpty = fullModel.keysAndNetwork.isEmpty(),
			onMenu = {
				menuNavController.navigate(KeySetDetailsMenuSubgraph.keys_menu)
			},
			onShowPublicKey = { title: String, key: String ->
				menuNavController.navigate("${KeySetDetailsMenuSubgraph.keys_public_key}/$title/$key")
			},
			onFilterClicked = {
				menuNavController.navigate(KeySetDetailsMenuSubgraph.network_filter)
			}
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
						menuNavController.navigate(KeySetDetailsMenuSubgraph.export)
					},
					onBackupClicked = {
						menuNavController.navigate(KeySetDetailsMenuSubgraph.backup) {
							popUpTo(KeySetDetailsMenuSubgraph.empty)
						}
					},
					onCancel = {
						menuNavController.popBackStack()
					},
					onDeleteClicked = {
						menuNavController.navigate(KeySetDetailsMenuSubgraph.keys_menu_delete_confirm) {
							popUpTo(KeySetDetailsMenuSubgraph.empty)
						}
					},
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
					onCancel = closeAction,
				)
			}
		}
		composable(KeySetDetailsMenuSubgraph.backup) {
			//preconditions
			val backupModel = fullModel.toSeedBackupModel()
			if (backupModel == null) {
				submitErrorState(
					"navigated to backup model but without root in KeySet " +
						"it's impossible to backup"
				)
				closeAction()
			} else {
				//content
				KeySetBackupFullOverlayBottomSheet(
					model = backupModel,
					getSeedPhraseForBackup = ::getSeedPhraseForBackup,
					onClose = closeAction,
				)
			}
		}
		composable(KeySetDetailsMenuSubgraph.export) {
			val selected = remember { mutableStateOf(setOf<String>()) }
			val isResultState = remember { mutableStateOf(false) }

			if (!isResultState.value) {
				BottomSheetWrapperRoot(onClosedAction = closeAction) {
					KeySetDetailsMultiselectBottomSheet(
						model = fullModel,
						selected = selected,
						onClose = closeAction,
						onExportSelected = {
							isResultState.value = true
						},
						onExportAll = {
							selected.value =
								fullModel.keysAndNetwork.map { it.key.addressKey }.toSet()
							isResultState.value = true
						},
					)
				}
			} else {
				BottomSheetWrapperRoot(onClosedAction = closeAction) {
					KeySetDetailsExportResultBottomSheet(
						model = fullModel,
						selectedKeys = selected.value,
						onClose = closeAction,
					)
				}
			}
		}
	}
}


private object KeySetDetailsMenuSubgraph {
	const val empty = "keys_menu_empty"
	const val keys_menu = "keys_menu"
	const val keys_menu_delete_confirm = "keys_menu_delete_confirm"
	const val network_filter = "keys_network_filters"
	const val backup = "keyset_details_backup"
	const val keys_public_key = "keys_public_key"
	const val export = "export_multiselect"
}

private const val ARGUMENT_PUBLIC_KEY_TITLE = "ARGUMENT_PUBLIC_KEY_TITLE"
private const val ARGUMENT_PUBLIC_KEY_VALUE = "ARGUMENT_PUBLIC_KEY_VALUE"

