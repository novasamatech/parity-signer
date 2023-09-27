package io.parity.signer.screens.settings.networks.details

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Modifier
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavController
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import androidx.navigation.navArgument
import io.parity.signer.domain.Callback
import io.parity.signer.screens.error.handleErrorAppState
import io.parity.signer.screens.settings.SettingsNavSubgraph
import io.parity.signer.screens.settings.networks.details.menu.ConfirmRemoveMetadataBottomSheet
import io.parity.signer.screens.settings.networks.details.menu.ConfirmRemoveNetworkBottomSheet
import io.parity.signer.screens.settings.networks.details.menu.NetworkDetailsMenuGeneral
import io.parity.signer.ui.BottomSheetWrapperRoot
import io.parity.signer.ui.mainnavigation.CoreUnlockedNavSubgraph
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking


@Composable
fun NetworkDetailsSubgraph(
	networkKey: String,
	navController: NavController,
) {
	val vm: NetworkDetailsViewModel = viewModel()

	val model = runBlocking {
		vm.getNetworkDetails(networkKey).handleErrorAppState(navController)
	} ?: return

	val coroutineScope = rememberCoroutineScope()
	val menuController = rememberNavController()

	Box(modifier = Modifier.statusBarsPadding()) {
		NetworkDetailsScreen(
			model = model,
			onBack = navController::popBackStack,
			onMenu = { menuController.navigate(NetworkDetailsMenuSubgraph.menu) },
			onSignMetadata = { metadataSpecVersion ->
				navController.navigate(
					SettingsNavSubgraph.SignMetadataSpecs.destination(
						networkKey,
						metadataSpecVersion
					)
				)
				menuController.popBackStack()
			},
			onRemoveMetadataCallback = { metadataSpecVersion ->
				menuController.navigate(
					NetworkDetailsMenuSubgraph.MetadataDeleteConfirm.destination(
						metadataSpecVersion
					)
				)
			},
			onAddNetwork = {
				navController.navigate(CoreUnlockedNavSubgraph.camera)
				menuController.popBackStack()
			},
		)
	}
	val closeAction: Callback = {
		menuController.popBackStack()
	}
	//bottom sheets
	NavHost(
		navController = menuController,
		startDestination = NetworkDetailsMenuSubgraph.empty,
	) {
		composable(NetworkDetailsMenuSubgraph.empty) {
			//no menu - Spacer element so when other part shown there won't
			// be an appearance animation from top left part despite there shouldn't be
			Spacer(modifier = Modifier.fillMaxSize(1f))
		}
		composable(NetworkDetailsMenuSubgraph.menu) {
			BottomSheetWrapperRoot(onClosedAction = closeAction) {
				NetworkDetailsMenuGeneral(
					onSignNetworkSpecs = {
						navController.navigate(
							SettingsNavSubgraph.SignNetworkSpecs.destination(
								networkKey,
							)
						)
						menuController.popBackStack()
					},
					onDeleteClicked = {
						menuController.navigate(NetworkDetailsMenuSubgraph.networkDeleteConfirm) {
							popUpTo(0)
						}
					},
					onCancel = closeAction,
				)
			}
		}
		composable(NetworkDetailsMenuSubgraph.networkDeleteConfirm) {
			BottomSheetWrapperRoot(onClosedAction = closeAction) {
				ConfirmRemoveNetworkBottomSheet(
					onRemoveNetwork = {
						coroutineScope.launch {
							val result = vm.removeNetwork(networkKey)
							result.handleErrorAppState(navController)?.let {
								navController.popBackStack()
							}
						}
						closeAction()
					},
					onCancel = closeAction,
				)
			}
		}
		composable(
			route = NetworkDetailsMenuSubgraph.MetadataDeleteConfirm.route,
			arguments = listOf(
				navArgument(NetworkDetailsMenuSubgraph.MetadataDeleteConfirm.metaSpecVersion) {
					type = NavType.StringType
				}
			),
		) {
			val versionSpec =
				it.arguments?.getString(NetworkDetailsMenuSubgraph.MetadataDeleteConfirm.metaSpecVersion)!!
			BottomSheetWrapperRoot(onClosedAction = closeAction) {
				ConfirmRemoveMetadataBottomSheet(
					onRemoveMetadata = {
						coroutineScope.launch {
							val result = vm.removeNetworkMetadata(networkKey, versionSpec)
							result.handleErrorAppState(navController)?.let {
								navController.popBackStack()
							}
						}
						closeAction()
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
	const val networkDeleteConfirm = "networkdetails_deleteConfirm"

	object MetadataDeleteConfirm {
		internal const val metaSpecVersion = "metaSpecVersion"
		private const val baseRoute = "networkdetails_metadata_deleteConfirm"
		const val route = "$baseRoute/{$metaSpecVersion}"
		fun destination(metaSpecVersion: String) = "$baseRoute/${metaSpecVersion}"
	}
}
