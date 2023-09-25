package io.parity.signer.screens.keysets

import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.State
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.components.exposesecurity.ExposedAlert
import io.parity.signer.domain.NetworkState
import io.parity.signer.screens.error.handleErrorAppState
import io.parity.signer.ui.mainnavigation.CoreUnlockedNavSubgraph


@Composable
fun KeySetsListScreenSubgraph(
	navController: NavController,
) {
	val vm: KeySetsViewModel = viewModel()

	val model = vm.keySetModel.collectAsStateWithLifecycle()
	val networkState: State<NetworkState?> =
		vm.networkState.collectAsStateWithLifecycle()

	LaunchedEffect(Unit) {
		vm.updateKeySetModel()
	}

	val menuNavController = rememberNavController()

	val modelValue = model.value.handleErrorAppState(navController) ?: return

	KeySetsListScreenFull(
		model = modelValue,
		navController = navController,
		onSelectSeed = { seedname ->
			navController.navigate(
				CoreUnlockedNavSubgraph.KeySetDetails.destination(
					seedName = seedname
				)
			)
		},
		onExposedShow = {
			menuNavController.navigate(KeySetsMenuSubgraph.exposed_shield_alert) {
				popUpTo(KeySetsMenuSubgraph.empty)
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


	//bottoms sheets
	NavHost(
		navController = menuNavController,
		startDestination = KeySetsMenuSubgraph.empty,
	) {
		composable(KeySetsMenuSubgraph.empty) {}//no menu
		composable(KeySetsMenuSubgraph.exposed_shield_alert) {
			ExposedAlert(navigateBack = { menuNavController.popBackStack() })
		}
	}

}

private object KeySetsMenuSubgraph {
	const val empty = "keysets_menu_empty"
	const val exposed_shield_alert = "keysets_exposed_shield_alert"
}
