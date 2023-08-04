package io.parity.signer.screens.keysetdetails

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.ui.Modifier
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.domain.*
import io.parity.signer.domain.storage.removeSeed
import io.parity.signer.screens.keysetdetails.backup.KeySetBackupFullOverlayBottomSheet
import io.parity.signer.screens.keysetdetails.backup.toSeedBackupModel
import io.parity.signer.screens.keysetdetails.export.KeySetDetailsExportScreenFull

@Composable
fun KeySetDetailsNavSubgraph(
    model: KeySetDetailsModel,
    rootNavigator: Navigator,
    networkState: State<NetworkState?>, //for shield icon
    singleton: SharedViewModel,
) {
	val navController = rememberNavController()
	NavHost(
		navController = navController,
		startDestination = KeySetDetailsNavSubgraph.home,
	) {

		composable(KeySetDetailsNavSubgraph.home) {
			KeySetDetailsScreenSubgraph(
				fullModel = model,
				navigator = rootNavigator,
				navController = navController,
				networkState = networkState,
				onRemoveKeySet = {
					val root = model.root
					if (root != null) {
						singleton.removeSeed(root.seedName)
					} else {
						//todo key details check if this functions should be disabled in a first place
						submitErrorState("came to remove key set but root key is not available")
					}
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

internal object KeySetDetailsNavSubgraph {
	const val home = "keyset_details_home"
	const val multiselect = "keyset_details_multiselect"
}
