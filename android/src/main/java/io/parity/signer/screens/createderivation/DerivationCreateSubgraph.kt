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
import io.parity.signer.domain.Callback
import io.parity.signer.domain.Navigator
import io.parity.signer.screens.createderivation.derivationsubscreens.*
import io.parity.signer.screens.createderivation.help.DerivationKeysHelpBottomSheet
import io.parity.signer.screens.createderivation.help.DerivationMethodsHelpBottomSheet
import io.parity.signer.screens.createderivation.help.DerivationPathHelpBottomSheet
import io.parity.signer.ui.BottomSheetWrapperRoot
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
		startDestination = DerivationCreateSubgraph.home,
	) {
		composable(DerivationCreateSubgraph.home) {
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
				onNetworkHelp = {},//todo dmitry implement.
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
	}
}

private object DerivationCreateSubgraph {
	const val home = "derivation_creation_home"
	const val path = "derivation_creation_path"
}

private object PathDerivationSheetsSubGraph {
	const val empty = "derivation_creation_path_sheets_empty"
	const val help = "derivation_creation_path_sheets_help"
	const val confirmation = "derivation_creation_confirmation"
}
