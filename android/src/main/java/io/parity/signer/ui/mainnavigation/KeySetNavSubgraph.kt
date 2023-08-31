package io.parity.signer.ui.mainnavigation

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import androidx.navigation.navArgument
import io.parity.signer.domain.Navigator
import io.parity.signer.domain.SharedViewModel
import io.parity.signer.domain.storage.removeSeed
import io.parity.signer.domain.submitErrorState
import io.parity.signer.domain.toKeySetDetailsModel
import io.parity.signer.screens.keydetails.KeyDetailsPublicKeyScreen
import io.parity.signer.screens.keydetails.KeyDetailsScreenSubgraph
import io.parity.signer.screens.keysetdetails.KeySetDetailsScreenSubgraph
import io.parity.signer.screens.keysets.KeySetsScreenSubgraph
import io.parity.signer.screens.keysets.create.NewKeysetStepSubgraph
import io.parity.signer.uniffi.ErrorDisplayed
import io.parity.signer.uniffi.keysBySeedName

@Composable
fun KeySetNavSubgraph(
	rootNavigator: Navigator,
	singleton: SharedViewModel,
) {

	val navController = rememberNavController()
	NavHost(
		navController = navController,
		startDestination = KeySetNavSubgraph.keySetList,
	) {
		composable(KeySetNavSubgraph.keySetList) {
			Box(modifier = Modifier.statusBarsPadding()) {
				KeySetsScreenSubgraph(
					rootNavigator = rootNavigator,
					navController = navController,
				)
			}
		}
		composable(
			route = KeySetNavSubgraph.KeySetDetails.route,
			arguments = listOf(
				navArgument(KeySetNavSubgraph.KeySetDetails.seedNameArg) {
					type = NavType.StringType
					defaultValue = null
				}
			)
		) {
			val seedName =
				it.arguments?.getString(KeySetNavSubgraph.KeySetDetails.seedNameArg)
			val model = remember {
				try {
					//todo dmitry export this to vm and handle errors - open default for example
					keysBySeedName(seedName!!).toKeySetDetailsModel()
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
		composable(KeySetNavSubgraph.newKeySet) {
			NewKeysetStepSubgraph(
				navController = navController,
			)
		}
		composable(KeySetNavSubgraph.recoverKeySet) {
			//todo dmitry implement
		}
		composable(KeySetNavSubgraph.keydetails) {

			KeyDetailsScreenSubgraph()

			//todo dmitry implement
			Box(modifier = Modifier.statusBarsPadding()) {
			screenData.f?.toKeyDetailsModel()?.let { model ->
				KeyDetailsPublicKeyScreen(
					model = model,
					rootNavigator = rootNavigator,
				)
			}
				?: run {
					submitErrorState("key details clicked for non existing key details content")
					rootNavigator.backAction()
				}
			}
		}
	}
}

internal object KeySetNavSubgraph {
	const val keySetList = "keyset_list"
	const val newKeySet = "keyset_details_new_keyset"
	const val recoverKeySet = "keyset_recover_flow"
	const val keydetails = "keyset_key_details"

	object KeySetDetails {
		internal const val seedNameArg = "title"
		private const val baseRoute = "keyset_details_home"
		const val route = "$baseRoute?{$seedNameArg}"
		fun destination(seedName: String) = "$baseRoute?${seedName}"
	}
}
