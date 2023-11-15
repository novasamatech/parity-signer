package io.parity.signer.screens.createderivation

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.imePadding
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalSoftwareKeyboardController
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewModelScope
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.domain.Callback
import io.parity.signer.screens.createderivation.derivationsubscreens.DerivationCreateConfirmBottomSheet
import io.parity.signer.screens.createderivation.derivationsubscreens.DerivationPathScreen
import io.parity.signer.screens.createderivation.derivationsubscreens.DeriveKeyNetworkSelectScreen
import io.parity.signer.screens.createderivation.help.DerivationMethodsHelpBottomSheet
import io.parity.signer.screens.settings.networks.helper.networkHelpersSubgraph
import io.parity.signer.ui.BottomSheetWrapperRoot
import kotlinx.coroutines.launch


@OptIn(ExperimentalComposeUiApi::class)
@Composable
fun DerivationCreateSubgraph(
	onBack: Callback,
	onOpenCamera: Callback,
	seedName: String,
) {

	val deriveViewModel: DerivationCreateViewModel = viewModel()
	deriveViewModel.setInitValues(seedName)
	val context = LocalContext.current
	val path = deriveViewModel.path.collectAsStateWithLifecycle()

	val navController = rememberNavController()
	NavHost(
		navController = navController,
		startDestination = DerivationCreateSubgraph.select_network,
	) {
		composable(DerivationCreateSubgraph.select_network) {
			DeriveKeyNetworkSelectScreen(
				networks = deriveViewModel.getAllNetworks(),
				preselectd = null,
				onClose = {
					deriveViewModel.resetState()
					onBack()
				},
				onAdvancePath = { newNetwork ->
					deriveViewModel.updateSelectedNetwork(newNetwork)
					navController.navigate(DerivationCreateSubgraph.path)
				},
				onFastCreate = { newNetwork ->
					deriveViewModel.viewModelScope.launch {
						deriveViewModel.fastCreateDerivationForNetwork(newNetwork, context)
						onBack()
					}
				},
				onNetworkHelp = { navController.navigate(DerivationCreateSubgraph.network_help) },
				modifier = Modifier
					.statusBarsPadding()
					.background(MaterialTheme.colors.background)
			)
		}
		composable(DerivationCreateSubgraph.path) {
			val menuNavController = rememberNavController()
			val keyboardController = LocalSoftwareKeyboardController.current
			DerivationPathScreen(
				initialPath = path.value,
				onDerivationHelp = {
					keyboardController?.hide()
					menuNavController.navigate(PathDerivationSheetsSubGraph.help)
				},
				onClose = { navController.popBackStack() },
				onDone = { newPath ->
					deriveViewModel.updatePath(newPath)
					keyboardController?.hide()
					menuNavController.navigate(PathDerivationSheetsSubGraph.confirmation)
				},
				validator = deriveViewModel::checkPath,
				modifier = Modifier
					.statusBarsPadding()
					.background(MaterialTheme.colors.background)
					.imePadding(),
			)
			//bottom sheets
			NavHost(
				navController = menuNavController,
				startDestination = PathDerivationSheetsSubGraph.empty,
			) {
				val closeAction: Callback = { menuNavController.popBackStack() }
				composable(PathDerivationSheetsSubGraph.empty) {
					//no menu - Spacer element so when other part shown there won't
					// be an appearance animation from top left part despite there shouldn't be
					Spacer(modifier = Modifier.fillMaxSize(1f))
				}
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
									onBack()
								}
							},
						)
					}
				}
			}
		}
		networkHelpersSubgraph(
			routePath = DerivationCreateSubgraph.network_help,
			onScanClicked = onOpenCamera,
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
