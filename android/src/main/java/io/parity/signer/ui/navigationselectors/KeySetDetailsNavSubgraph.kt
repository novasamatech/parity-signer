package io.parity.signer.ui.navigationselectors

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.ui.Modifier
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.models.*
import io.parity.signer.screens.keysets.KeySetDetailsMultiselectScreen
import io.parity.signer.screens.keysets.details.KeySetDetailsScreenFull

@Composable
fun KeySetDetailsNavSubgraph(
	model: KeySetDetailsModel,
	rootNavigator: Navigator,
	alertState: State<AlertState?>, //for shield icon
	sigleton: SignerDataModel,
) {
	val navController = rememberNavController()
	NavHost(
		navController = navController,
		startDestination = KeySetDetailsNavSubgraph.home,
	) {

		composable(KeySetDetailsNavSubgraph.home) {
			KeySetDetailsScreenFull(
				model = model,
				navigator = rootNavigator,
				alertState = alertState,
				onRemoveKeySet = {
					sigleton.removeSeed(model.root.seedName)
				}
			)
		}
		composable(KeySetDetailsNavSubgraph.multiselect) {
			Box(modifier = Modifier.statusBarsPadding()) {
				KeySetDetailsMultiselectScreen(
					model = model,
					navigator = rootNavigator,
					alertState = alertState,
				)
			}
		}
	}
}

object KeySetDetailsNavSubgraph {
	const val home = "keyset_details_home"
	const val multiselect = "keyset_details_multiselect"
}
