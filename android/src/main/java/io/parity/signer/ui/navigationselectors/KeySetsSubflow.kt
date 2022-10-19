package io.parity.signer.ui.navigationselectors

import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.rememberNavController
import io.parity.signer.models.AlertState
import io.parity.signer.models.Navigator
import io.parity.signer.screens.keysets.KeySetsSelectViewModel

/**
 * Navigation Subgraph with compose nav controller for those Key Set screens which are not part of general
 * Rust-controlled navigation
 */
@Composable
fun KeySetsSubflow(
	model: KeySetsSelectViewModel,
	rootNavigator: Navigator,
	alertState: State<AlertState?>, //for shield icon
) {
	val navController = rememberNavController()
	NavHost(
		navController = navController,
		startDestination = "home",
	) {

	}
}
