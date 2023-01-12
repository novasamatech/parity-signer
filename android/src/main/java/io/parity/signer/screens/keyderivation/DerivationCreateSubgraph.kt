package io.parity.signer.screens.keyderivation

import androidx.compose.runtime.Composable
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.screens.keysetdetails.KeySetDetailsNavSubgraph


@Composable
fun DerivationCreateSubgraph() {
	val navController = rememberNavController()
	NavHost(
		navController = navController,
		startDestination = KeySetDetailsNavSubgraph.home,
	) {
		composable(DerivationCreateSubgraph.home) {

		}
		composable(DerivationCreateSubgraph.path) {

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
