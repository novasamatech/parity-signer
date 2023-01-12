package io.parity.signer.screens.keyderivation

import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.models.Navigator
import io.parity.signer.screens.keyderivation.derivationsubscreens.DerivationPathScreen
import io.parity.signer.screens.keyderivation.derivationsubscreens.DeriveKeyBaseScreen
import io.parity.signer.screens.keysetdetails.KeySetDetailsNavSubgraph


@Composable
fun DerivationCreateSubgraph(
	rootNavigator: Navigator,
	seedName: String,
	networkSpecsKey: String,
) {

	val deriveViewModel: DerivationCreateViewModel = viewModel()
	deriveViewModel.setInitValues(seedName, networkSpecsKey, rootNavigator)

	val path = deriveViewModel.path.collectAsState()

	val navController = rememberNavController()
	NavHost(
		navController = navController,
		startDestination = KeySetDetailsNavSubgraph.home,
	) {
		composable(DerivationCreateSubgraph.home) {
			DeriveKeyBaseScreen(
				onClose = { rootNavigator.backAction() },
				onNetworkSelectClicked = {},//todo derivations,
				onDerivationHelpClicked = {},
				onPathClicked = { navController.navigate(DerivationCreateSubgraph.path) },
			)
		}
		composable(DerivationCreateSubgraph.path) {
			DerivationPathScreen(
				initialPath = path.value,
				onDerivationHelp = { /*TODO*/ },
				onClose = { navController.navigate(DerivationCreateSubgraph.home) },
				onDone = { newPath ->
					deriveViewModel.updatePath(newPath)
					navController.navigate(DerivationCreateSubgraph.home)
				}
			)
		}
		composable(DerivationCreateSubgraph.confirmation) {

		}
	}
}

internal object DerivationCreateSubgraph {
	const val home = "derivation_creation_home"
	const val path = "derivation_creation_path"
	const val confirmation = "derivation_creation_confirmation"
}

internal object Basic
