package io.parity.signer.ui.navigationselectors

import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.models.*
import io.parity.signer.screens.keysetdetails.KeySetDetailsScreenFull
import io.parity.signer.screens.keysetdetails.export.KeySetDetailsExportScreenFull

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
				navController = navController,
				onRemoveKeySet = {
					sigleton.removeSeed(model.root.seedName)
				},
			)
		}
		composable(KeySetDetailsNavSubgraph.multiselect) {
			KeySetDetailsExportScreenFull(
				model = model,
				onClose = { navController.navigate(KeySetDetailsNavSubgraph.home) },
			)
		}
	}
}

object KeySetDetailsNavSubgraph {
	const val home = "keyset_details_home"
	const val multiselect = "keyset_details_multiselect"
}
