package io.parity.signer.screens.keysetdetails

import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.runtime.remember
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.domain.Navigator
import io.parity.signer.domain.NetworkState
import io.parity.signer.domain.SharedViewModel
import io.parity.signer.domain.storage.removeSeed
import io.parity.signer.domain.submitErrorState
import io.parity.signer.domain.toKeySetDetailsModel
import io.parity.signer.screens.keysets.create.NewKeysetStepSubgraph
import io.parity.signer.uniffi.ErrorDisplayed
import io.parity.signer.uniffi.keysBySeedName

@Composable
fun KeySetDetailsNavSubgraph(
	seedName: String,
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
			val model = remember {
				//todo dmitry pass it inside of this screen?
				try {
					keysBySeedName(seedName).toKeySetDetailsModel()
				} catch (e: ErrorDisplayed) {
					rootNavigator.backAction()
					submitErrorState("unexpected error in keysBySeedName $e")
					null
				}
			}
			model?.let {
				KeySetDetailsScreenSubgraph(
					fullModel = model,
					navigator = rootNavigator,
					navController = navController,
					networkState = networkState,
					onBack = { rootNavigator.backAction() },
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
		}
		composable(KeySetDetailsNavSubgraph.newKeySet) {
			NewKeysetStepSubgraph(
				navController = navController,
			)
		}
	}
}

internal object KeySetDetailsNavSubgraph {
	const val home = "keyset_details_home"
	const val newKeySet = "keyset_details_new_keyset"
}
