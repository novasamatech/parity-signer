package io.parity.signer.screens.createderivation

import androidx.compose.foundation.layout.imePadding
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.lifecycle.viewModelScope
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.components.panels.CameraParentScreen
import io.parity.signer.components.panels.CameraParentSingleton
import io.parity.signer.domain.Callback
import io.parity.signer.domain.Navigator
import io.parity.signer.screens.createderivation.derivationsubscreens.DerivationCreateConfirmBottomSheet
import io.parity.signer.screens.createderivation.derivationsubscreens.DerivationPathScreen
import io.parity.signer.screens.createderivation.derivationsubscreens.DeriveKeyNetworkSelectScreen
import io.parity.signer.screens.createderivation.help.DerivationMethodsHelpBottomSheet
import io.parity.signer.screens.settings.networks.helper.networkHelpersSubgraph
import io.parity.signer.ui.BottomSheetWrapperRoot
import io.parity.signer.uniffi.Action
import kotlinx.coroutines.launch


@Composable
fun DerivationCreateSubgraph(
	rootNavigator: Navigator,
	seedName: String,
) {

	val deriveViewModel: DerivationCreateViewModel = viewModel()
	deriveViewModel.setInitValues(seedName, rootNavigator)
	val context = LocalContext.current
	val path = deriveViewModel.path.collectAsState()

	val navController = rememberNavController()
	NavHost(
		navController = navController,
		startDestination = DerivationCreateSubgraph.select_network,
	) {
		composable(DerivationCreateSubgraph.select_network) {
			DeriveKeyNetworkSelectScreen(
				networks = deriveViewModel.allNetworks,
				onClose = {
					deriveViewModel.resetState()
					rootNavigator.backAction()
				},
				onNetworkSelect = { newNetwork ->
					deriveViewModel.updateSelectedNetwork(newNetwork)
					navController.navigate(DerivationCreateSubgraph.path)
				},
				onNetworkHelp = { navController.navigate(DerivationCreateSubgraph.network_help) },
				modifier = Modifier.statusBarsPadding()
			)
		}
		composable(DerivationCreateSubgraph.path) {
			val menuNavController = rememberNavController()

			DerivationPathScreen(
				initialPath = path.value,
				onDerivationHelp = {
					menuNavController.navigate(PathDerivationSheetsSubGraph.help)
				},
				onClose = { navController.popBackStack() },
				onDone = { newPath ->
					deriveViewModel.updatePath(newPath)
					menuNavController.navigate(PathDerivationSheetsSubGraph.confirmation)
				},
				validator = deriveViewModel::checkPath,
				modifier = Modifier
					.statusBarsPadding()
					.imePadding(),
			)
			//bottom sheets
			NavHost(
				navController = menuNavController,
				startDestination = PathDerivationSheetsSubGraph.empty,
			) {
				val closeAction: Callback = { menuNavController.popBackStack() }
				composable(PathDerivationSheetsSubGraph.empty) {}
				composable(PathDerivationSheetsSubGraph.help) {
					BottomSheetWrapperRoot(onClosedAction = closeAction) {
						DerivationMethodsHelpBottomSheet(
							onClose = closeAction,
						)
					}
				}
				composable(PathDerivationSheetsSubGraph.confirmation) {
					BottomSheetWrapperRoot(onClosedAction = closeAction) {
						DerivationCreateConfirmBottomSheet(
							path = path.value,
							onDone = {
								deriveViewModel.viewModelScope.launch {
									deriveViewModel.proceedCreateKey(
										context
									)
									closeAction()
									rootNavigator.backAction()
								}
							},
						)
					}
				}
			}
		}
		networkHelpersSubgraph(
			routePath = DerivationCreateSubgraph.network_help,
			onScanClicked = {
				CameraParentSingleton.lastPossibleParent =
					CameraParentScreen.CreateDerivationScreen(seedName)
				rootNavigator.backAction()
				rootNavigator.navigate(Action.NAVBAR_SCAN)
			},
			navController = navController,
		)
	}
}

private object DerivationCreateSubgraph {
	const val select_network = "derivation_creation_select_network"
	const val path = "derivation_creation_path"
	const val network_help = "network_help_screens"
}

private object PathDerivationSheetsSubGraph {
	const val empty = "derivation_creation_path_sheets_empty"
	const val help = "derivation_creation_path_sheets_help"
	const val confirmation = "derivation_creation_confirmation"
}
