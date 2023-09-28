package io.parity.signer.ui.mainnavigation

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import androidx.navigation.navArgument
import io.parity.signer.domain.addVaultLogger
import io.parity.signer.screens.createderivation.DerivationCreateSubgraph
import io.parity.signer.screens.error.errorStateDestination
import io.parity.signer.screens.keydetails.KeyDetailsScreenSubgraph
import io.parity.signer.screens.keysetdetails.keySetDetailsDestination
import io.parity.signer.screens.keysets.KeySetsListScreenSubgraph
import io.parity.signer.screens.keysets.create.NewKeysetSubgraph
import io.parity.signer.screens.keysets.restore.KeysetRecoverSubgraph
import io.parity.signer.screens.scan.ScanNavSubgraph
import io.parity.signer.screens.settings.networks.helper.networkHelpersCoreSubgraph
import io.parity.signer.screens.settings.settingsFullSubgraph

@Composable
fun CoreUnlockedNavSubgraph() {

	val navController = rememberNavController().apply { addVaultLogger() }
	NavHost(
		navController = navController,
		startDestination = CoreUnlockedNavSubgraph.keySetList,
	) {
		composable(CoreUnlockedNavSubgraph.keySetList) {
			Box(modifier = Modifier.statusBarsPadding()) {
				KeySetsListScreenSubgraph(
					navController = navController,
				)
			}
		}
		keySetDetailsDestination(navController)
		composable(CoreUnlockedNavSubgraph.newKeySet) {
			NewKeysetSubgraph(
				coreNavController = navController,
			)
		}
		composable(CoreUnlockedNavSubgraph.recoverKeySet) {
			KeysetRecoverSubgraph(
				coreNavController = navController
			)
		}
		composable(
			route = CoreUnlockedNavSubgraph.KeyDetails.route,
			arguments = listOf(
				navArgument(CoreUnlockedNavSubgraph.KeyDetails.argKeyAddr) {
					type = NavType.StringType
				},
				navArgument(CoreUnlockedNavSubgraph.KeyDetails.argKeySpec) {
					type = NavType.StringType
				},
			)
		) {
			val keyAddr =
				it.arguments?.getString(CoreUnlockedNavSubgraph.KeyDetails.argKeyAddr)!!
			val keySpec =
				it.arguments?.getString(CoreUnlockedNavSubgraph.KeyDetails.argKeySpec)!!
			KeyDetailsScreenSubgraph(
				navController = navController,
				keyAddr = keyAddr,
				keySpec = keySpec
			)
		}
		composable(
			route = CoreUnlockedNavSubgraph.NewDerivedKey.route,
			arguments = listOf(
				navArgument(CoreUnlockedNavSubgraph.NewDerivedKey.seedNameArg) {
					type = NavType.StringType
				}
			),
		) {
			val seedName =
				it.arguments?.getString(CoreUnlockedNavSubgraph.KeySetDetails.seedNameOptionalArg)!!

			DerivationCreateSubgraph(
				onBack = { navController.popBackStack() },
				onOpenCamera = { navController.navigate(CoreUnlockedNavSubgraph.camera) },
				seedName = seedName,
			)
		}
		composable(CoreUnlockedNavSubgraph.camera) {
			ScanNavSubgraph(
				onCloseCamera = {
					navController.popBackStack()
				},
				openKeySet = { seedName ->
					navController.navigate(
						CoreUnlockedNavSubgraph.KeySetDetails.destination(
							seedName = seedName
						)
					)
				}
			)
		}
		settingsFullSubgraph(
			navController = navController,
		)
		networkHelpersCoreSubgraph(
			navController = navController,
		)
		errorStateDestination(
			navController = navController,
		)
	}
}

object CoreUnlockedNavSubgraph {
	const val keySetList = "core_keyset_list"
	const val newKeySet = "core_new_keyset"
	const val recoverKeySet = "keyset_recover_flow"
	const val camera = "unlocked_camera"

	object KeySetDetails {
		internal const val seedNameOptionalArg = "seed_name_arg"
		private const val baseRoute = "core_keyset_details_home"
		const val route = "$baseRoute/{$seedNameOptionalArg}"
		fun destination(seedName: String?) = "$baseRoute/${seedName.orEmpty()}"
	}

	object NewDerivedKey {
		internal const val seedNameArg = "seed_name_arg"
		private const val baseRoute = "core_new_keyset"
		const val route = "$baseRoute/{${seedNameArg}}"
		fun destination(seedName: String) = "$baseRoute/${seedName}"
	}

	object KeyDetails {
		internal const val argKeyAddr = "key_addr"
		internal const val argKeySpec = "key_spec"
		private const val baseRoute = "core_key_details_home"
		const val route = "$baseRoute/{$argKeyAddr}/{$argKeySpec}"
		fun destination(keyAddr: String, keySpec: String) =
			"$baseRoute/$keyAddr/$keySpec"
	}

	object ErrorScreen {
		internal const val argHeader = "key_header"
		internal const val argDescription = "key_descr"
		internal const val argVerbose = "key_verb"
		private const val baseRoute = "core_error_state"
		const val route = "$baseRoute/{$argHeader}/{$argDescription}/{$argVerbose}"
		fun destination(
			argHeader: String,
			argDescription: String,
			argVerbose: String
		) =
			"$baseRoute/$argHeader/$argDescription/$argVerbose"
	}

	const val settings = "core_settings_flow"
	const val networkHelpers = "network_helpers_path"
}
