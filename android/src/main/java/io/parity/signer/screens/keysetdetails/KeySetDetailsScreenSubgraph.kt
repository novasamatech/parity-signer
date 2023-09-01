package io.parity.signer.screens.keysetdetails

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
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
import io.parity.signer.components.exposesecurity.ExposedAlert
import io.parity.signer.domain.Callback
import io.parity.signer.domain.KeySetDetailsModel
import io.parity.signer.domain.Navigator
import io.parity.signer.domain.submitErrorState
import io.parity.signer.screens.keysetdetails.backup.KeySetBackupFullOverlayBottomSheet
import io.parity.signer.screens.keysetdetails.backup.toSeedBackupModel
import io.parity.signer.screens.keysetdetails.export.KeySetDetailsExportResultBottomSheet
import io.parity.signer.screens.keysetdetails.export.KeySetDetailsMultiselectBottomSheet
import io.parity.signer.screens.keysetdetails.filtermenu.NetworkFilterMenu
import io.parity.signer.ui.BottomSheetWrapperRoot
import io.parity.signer.ui.mainnavigation.CoreUnlockedNavSubgraph

@Composable
fun KeySetDetailsScreenSubgraph(
	fullModel: KeySetDetailsModel,
	navigator: Navigator,
	navController: NavController,
	onBack: Callback,
	onRemoveKeySet: Callback,
) {
	val menuNavController = rememberNavController()

	val keySetViewModel: KeySetDetailsViewModel = viewModel()
	val filteredModel =
		keySetViewModel.makeFilteredFlow(fullModel).collectAsStateWithLifecycle()

	val networkState = keySetViewModel.networkState.collectAsStateWithLifecycle()

	Box(Modifier.statusBarsPadding()) {
		KeySetDetailsScreenView(
			model = filteredModel.value,
			navigator = navigator,
			networkState = networkState,
			fullModelWasEmpty = fullModel.keysAndNetwork.isEmpty(),
			onExposedClicked = {
				menuNavController.navigate(KeySetDetailsMenuSubgraph.exposed_shield_alert) {
					popUpTo(KeySetDetailsMenuSubgraph.empty)
				}
			},
			onMenu = {
				menuNavController.navigate(KeySetDetailsMenuSubgraph.keys_menu)
			},
			onShowPublicKey = { title: String, key: String ->
				menuNavController.navigate(
					KeySetDetailsMenuSubgraph.KeysPublicKey.destination(
						title,
						key
					)
				)
			},
			onBack = onBack,
			onAddNewKey = {
				navController.navigate(CoreUnlockedNavSubgraph.newKeySet)
			},
			onFilterClicked = {
				menuNavController.navigate(KeySetDetailsMenuSubgraph.network_filter)
			},
			onOpenKey = { keyAddr: String, keySpecs: String ->
				navController.navigate(
					CoreUnlockedNavSubgraph.KeyDetails.destination(
						keyAddr = keyAddr,
						keySpec = keySpecs
					)
				)
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
					exposeConfirmAction = {
						menuNavController.navigate(KeySetDetailsMenuSubgraph.exposed_shield_alert) {
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
			route = KeySetDetailsMenuSubgraph.KeysPublicKey.route,
			arguments = listOf(
				navArgument(KeySetDetailsMenuSubgraph.KeysPublicKey.key_title_arg) {
					type = NavType.StringType
				},
				navArgument(KeySetDetailsMenuSubgraph.KeysPublicKey.key_valie_arg) {
					type = NavType.StringType
				}
			)
		) { backStackEntry ->
			val keyName =
				backStackEntry.arguments
					?.getString(KeySetDetailsMenuSubgraph.KeysPublicKey.key_title_arg)
					?: run {
						submitErrorState("mandatory parameter missing for KeySetDetailsMenuSubgraph.keys_public_key")
						""
					}
			val keyValue =
				backStackEntry.arguments
					?.getString(KeySetDetailsMenuSubgraph.KeysPublicKey.key_valie_arg)
					?: run {
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
					getSeedPhraseForBackup = keySetViewModel::getSeedPhrase,
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
		composable(KeySetDetailsMenuSubgraph.exposed_shield_alert) {
			ExposedAlert(navigateBack = { menuNavController.popBackStack() })
		}
	}
}


private object KeySetDetailsMenuSubgraph {
	const val empty = "keys_menu_empty"
	const val keys_menu = "keys_menu"
	const val keys_menu_delete_confirm = "keys_menu_delete_confirm"
	const val network_filter = "keys_network_filters"
	const val backup = "keyset_details_backup"
	const val export = "export_multiselect"
	const val exposed_shield_alert = "keys_exposed_shield_alert"

	object KeysPublicKey {
		internal const val key_title_arg = "ARGUMENT_PUBLIC_KEY_TITLE"
		internal const val key_valie_arg = "ARGUMENT_PUBLIC_KEY_VALUE"
		private const val baseRoute = "keys_public_key"
		const val route = "$baseRoute/{$key_title_arg}/{$key_valie_arg}"
		fun destination(keyTitle: String, keyValue: String) =
			"$baseRoute/$keyTitle/$keyValue"
	}
}

