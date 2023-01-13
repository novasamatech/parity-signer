package io.parity.signer.screens.keyderivation

import androidx.compose.foundation.layout.imePadding
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Modifier
import androidx.lifecycle.viewModelScope
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.models.Callback
import io.parity.signer.models.Navigator
import io.parity.signer.screens.keyderivation.derivationsubscreens.DerivationCreateConfirmBottomSheet
import io.parity.signer.screens.keyderivation.derivationsubscreens.DerivationPathScreen
import io.parity.signer.screens.keyderivation.derivationsubscreens.DeriveKeyBaseScreen
import io.parity.signer.screens.keyderivation.derivationsubscreens.NetworkSelectionBottomSheet
import io.parity.signer.screens.keyderivation.help.DerivationKeysHelpBottomSheet
import io.parity.signer.screens.keyderivation.help.DerivationMethodsHelpBottomSheet
import io.parity.signer.screens.keyderivation.help.DerivationPathHelpBottomSheet
import io.parity.signer.ui.BottomSheetWrapperRoot
import kotlinx.coroutines.launch


@Composable
fun DerivationCreateSubgraph(
	rootNavigator: Navigator,
	seedName: String,
	networkSpecsKey: String,
) {

	val deriveViewModel: DerivationCreateViewModel = viewModel()
	deriveViewModel.setInitValues(seedName, networkSpecsKey, rootNavigator)

	val path = deriveViewModel.path.collectAsState()
	val selectedNetwork = deriveViewModel.selectedNetwork.collectAsState()

	val navController = rememberNavController()
	NavHost(
		navController = navController,
		startDestination = DerivationCreateSubgraph.home,
	) {
		composable(DerivationCreateSubgraph.home) {
			val subNavController = rememberNavController()
			DeriveKeyBaseScreen(
				path = path.value,
				isPathValid = deriveViewModel.checkPath(path.value) == DerivationCreateViewModel.DerivationPathValidity.ALL_GOOD,
				selectedNetwork = selectedNetwork.value,
				onClose = { rootNavigator.backAction() },
				onNetworkSelectClicked = {
					subNavController.navigate(HomeDerivationSheetsSubGraph.networks)
				},
				onDerivationMenuHelpClicked = {
					subNavController.navigate(HomeDerivationSheetsSubGraph.derivationMenuHelp)
				},
				onDerivationPathHelpClicked = {
					subNavController.navigate(HomeDerivationSheetsSubGraph.derivationPathHelp)
				},
				onPathClicked = { navController.navigate(DerivationCreateSubgraph.path) },
				onCreateClicked = {
					subNavController.navigate(HomeDerivationSheetsSubGraph.confirmation)
				},
				modifier = Modifier.statusBarsPadding()
			)

			//bottom sheets
			NavHost(
				navController = subNavController,
				startDestination = HomeDerivationSheetsSubGraph.empty,
			) {
				val closeAction: Callback = {
					subNavController.popBackStack()
				}
				composable(HomeDerivationSheetsSubGraph.empty) {}
				composable(HomeDerivationSheetsSubGraph.networks) {
					BottomSheetWrapperRoot(onClosedAction = closeAction) {
						NetworkSelectionBottomSheet(
							networks = deriveViewModel.allNetworks,
							currentlySelectedNetwork = selectedNetwork.value,
							onClose = closeAction,
							onSelect = { newNetwork ->
								deriveViewModel.updateSelectedNetwork(newNetwork)
								closeAction()
							},
						)
					}
				}
				composable(HomeDerivationSheetsSubGraph.derivationMenuHelp) {
					BottomSheetWrapperRoot(onClosedAction = closeAction) {
						DerivationKeysHelpBottomSheet(
							onClose = closeAction,
						)
					}
				}
				composable(HomeDerivationSheetsSubGraph.derivationPathHelp) {
					BottomSheetWrapperRoot(onClosedAction = closeAction) {
						DerivationPathHelpBottomSheet(
							onClose = closeAction,
						)
					}
				}
				composable(HomeDerivationSheetsSubGraph.confirmation) {
					BottomSheetWrapperRoot(onClosedAction = closeAction) {
						DerivationCreateConfirmBottomSheet(
							path = path.value,
							onCancel = closeAction,
							onDone = {
								deriveViewModel.viewModelScope.launch { deriveViewModel.proceedCreateKey() }
								closeAction()
							},
						)
					}
				}
			}
		}

		composable(DerivationCreateSubgraph.path) {
			val subNavController = rememberNavController()

			DerivationPathScreen(
				initialPath = path.value,
				onDerivationHelp = {
					subNavController.navigate(PathDerivationSheetsSubGraph.help)
				},
				onClose = { navController.popBackStack() },
				onDone = { newPath ->
					deriveViewModel.updatePath(newPath)
					navController.popBackStack()
				},
				validator = deriveViewModel::checkPath,
				modifier = Modifier
					.statusBarsPadding()
					.imePadding(),
			)
			//bottom sheets
			NavHost(
				navController = subNavController,
				startDestination = PathDerivationSheetsSubGraph.empty,
			) {
				val closeAction: Callback = { subNavController.popBackStack() }
				composable(PathDerivationSheetsSubGraph.empty) {}
				composable(PathDerivationSheetsSubGraph.help) {
					BottomSheetWrapperRoot(onClosedAction = closeAction) {
						DerivationMethodsHelpBottomSheet(
							onClose = closeAction,
						)
					}
				}
			}
		}
	}
}

internal object DerivationCreateSubgraph {
	const val home = "derivation_creation_home"
	const val path = "derivation_creation_path"
}

private object HomeDerivationSheetsSubGraph {
	const val empty = "derivation_creation_basic_sheets_empty"
	const val networks = "derivation_creation_basic_sheets_networks"
	const val derivationMenuHelp =
		"derivation_creation_basic_sheets_derivationMenuHelp"
	const val derivationPathHelp =
		"derivation_creation_basic_sheets_derivationPathHelp"
	const val confirmation = "derivation_creation_confirmation"
}

private object PathDerivationSheetsSubGraph {
	const val empty = "derivation_creation_path_sheets_empty"
	const val help = "derivation_creation_path_sheets_help"
}
