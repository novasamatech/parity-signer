package io.parity.signer.screens.keysetdetails

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.components.exposesecurity.ExposedAlert
import io.parity.signer.domain.Callback
import io.parity.signer.domain.submitErrorState
import io.parity.signer.screens.error.handleErrorAppState
import io.parity.signer.screens.initial.WaitingScreen
import io.parity.signer.screens.keysetdetails.backup.KeySetBackupFullOverlayBottomSheet
import io.parity.signer.screens.keysetdetails.backup.toSeedBackupModel
import io.parity.signer.screens.keysetdetails.empty.NoKeySetEmptyWelcomeScreen
import io.parity.signer.screens.keysetdetails.export.KeySetDetailsExportResultBottomSheet
import io.parity.signer.screens.keysetdetails.export.KeySetDetailsMultiselectBottomSheet
import io.parity.signer.screens.keysetdetails.exportroot.KeySetRootExportBottomSheet
import io.parity.signer.screens.keysetdetails.filtermenu.NetworkFilterMenu
import io.parity.signer.screens.keysetdetails.seedselectmenu.SeedSelectMenuFull
import io.parity.signer.ui.BottomSheetWrapperRoot
import io.parity.signer.ui.mainnavigation.CoreUnlockedNavSubgraph
import kotlinx.coroutines.launch

/**
 * @param seedName is optional - null when we need to open last one
 */
@Composable
fun KeySetDetailsScreenSubgraph(
	originalSeedName: String?,
	navController: NavController,
) {
	val menuNavController = rememberNavController()
	val coroutineScope = rememberCoroutineScope()

	var seedName by rememberSaveable() {
		mutableStateOf(originalSeedName)
	}

	val keySetViewModel: KeySetDetailsViewModel = viewModel()

	LaunchedEffect(key1 = seedName) {
		keySetViewModel.feedModelForSeed(seedName)
	}
	LaunchedEffect(key1 = Unit) {
		//this is first screen - check db version here
		keySetViewModel.validateDbSchemaCorrect().handleErrorAppState(navController)
	}

	val filteredScreenModel =
		keySetViewModel.filteredScreenState.collectAsStateWithLifecycle()
	val networkState = keySetViewModel.networkState.collectAsStateWithLifecycle()

	when (val state =
		filteredScreenModel.value.handleErrorAppState(navController)) {

		KeySetDetailsScreenState.NoKeySets -> {
			NoKeySetEmptyWelcomeScreen(
				onExposedShow = {
					menuNavController.navigate(KeySetDetailsMenuSubgraph.exposed_shield_alert) {
						popUpTo(KeySetDetailsMenuSubgraph.empty)
					}
				},
				onNewKeySet = {
					navController.navigate(
						CoreUnlockedNavSubgraph.newKeySet
					)
				},
				onRecoverKeySet = {
					navController.navigate(
						CoreUnlockedNavSubgraph.recoverKeySet
					)
				},
				networkState = networkState,
			)
		}

		KeySetDetailsScreenState.LoadingState, null -> WaitingScreen()
		is KeySetDetailsScreenState.Data -> {

			Box(Modifier.statusBarsPadding()) {
				KeySetDetailsScreenView(
					model = state.filteredModel,
					navController = navController,
					networkState = networkState,
					fullModelWasEmpty = state.wasEmptyKeyset,
					onExposedClicked = {
						menuNavController.navigate(KeySetDetailsMenuSubgraph.exposed_shield_alert) {
							popUpTo(KeySetDetailsMenuSubgraph.empty)
						}
					},
					onMenu = {
						menuNavController.navigate(KeySetDetailsMenuSubgraph.keys_menu)
					},
					onAddNewDerivation = {
						navController.navigate(
							CoreUnlockedNavSubgraph.NewDerivedKey.destination(
								seedName = state.filteredModel.root.seedName
							)
						)
					},
					onSeedSelect = {
						menuNavController.navigate(KeySetDetailsMenuSubgraph.select_keyset)
					},
					onFilterClicked = {
						menuNavController.navigate(KeySetDetailsMenuSubgraph.network_filter)
					},
					onShowRoot = {
						menuNavController.navigate(KeySetDetailsMenuSubgraph.show_root)
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
				composable(KeySetDetailsMenuSubgraph.empty) {
					//no menu - Spacer element so when other part shown there won't
					// be an appearance animation from top left part despite there shouldn't be
					Spacer(modifier = Modifier.fillMaxSize(1f))
				}
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
				composable(KeySetDetailsMenuSubgraph.select_keyset) {
					SeedSelectMenuFull(
						coreNavController = navController,
						selectedSeed = state.filteredModel.root.seedName,
						onSelectSeed = { newSeed ->
							seedName = newSeed
							closeAction()
						},
						onClose = closeAction,
					)
				}
				composable(KeySetDetailsMenuSubgraph.keys_menu_delete_confirm) {
					BottomSheetWrapperRoot(onClosedAction = closeAction) {
						KeySetDeleteConfirmBottomSheet(
							onCancel = closeAction,
							onRemoveKeySet = {
								val root = state.filteredModel.root
								coroutineScope.launch {
									keySetViewModel.removeSeed(root)
										.handleErrorAppState(navController)
									closeAction()
								}
							},
						)
					}
				}
				composable(
					route = KeySetDetailsMenuSubgraph.show_root,
				) { backStackEntry ->
					BottomSheetWrapperRoot(onClosedAction = closeAction) {
						KeySetRootExportBottomSheet(
							model = state.filteredModel.root,
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
					val backupModel = state.filteredModel.toSeedBackupModel()
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
					val selected = rememberSaveable { mutableStateOf(setOf<String>()) }
					val isResultState = rememberSaveable { mutableStateOf(false) }

					if (!isResultState.value) {
						BottomSheetWrapperRoot(onClosedAction = closeAction) {
							KeySetDetailsMultiselectBottomSheet(
								model = state.filteredModel,
								selected = selected,
								onClose = closeAction,
								onExportSelected = {
									isResultState.value = true
								},
								onExportAll = {
									selected.value =
										state.filteredModel.keysAndNetwork.map { it.key.addressKey }
											.toSet()
									isResultState.value = true
								},
							)
						}
					} else {
						BottomSheetWrapperRoot(onClosedAction = closeAction) {
							KeySetDetailsExportResultBottomSheet(
								model = state.filteredModel,
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
	}
}


private object KeySetDetailsMenuSubgraph {
	const val empty = "keys_menu_empty"
	const val select_keyset = "keys_menu_select_keyset"
	const val keys_menu = "keys_menu"
	const val keys_menu_delete_confirm = "keys_menu_delete_confirm"
	const val network_filter = "keys_network_filters"
	const val backup = "keyset_details_backup"
	const val export = "export_multiselect"
	const val exposed_shield_alert = "keys_exposed_shield_alert"
	const val show_root = "keyset_root_export_and_base58"
}

