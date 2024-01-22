package io.parity.signer.ui.mainnavigation

import androidx.compose.animation.AnimatedContentTransitionScope
import androidx.compose.animation.EnterTransition
import androidx.compose.animation.ExitTransition
import androidx.compose.animation.core.tween
import androidx.compose.runtime.Composable
import androidx.navigation.NavHostController
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.navArgument
import io.parity.signer.screens.createderivation.DerivationCreateSubgraph
import io.parity.signer.screens.error.errorStateDestination
import io.parity.signer.screens.keydetails.KeyDetailsScreenSubgraph
import io.parity.signer.screens.keysetdetails.keySetDetailsDestination
import io.parity.signer.screens.keysets.create.NewKeysetSubgraph
import io.parity.signer.screens.keysets.restore.KeysetRecoverSubgraph
import io.parity.signer.screens.scan.ScanNavSubgraph
import io.parity.signer.screens.settings.networks.helper.networkHelpersCoreSubgraph
import io.parity.signer.screens.settings.settingsFullSubgraph

@Composable
fun CoreUnlockedNavSubgraph(navController: NavHostController) {

	NavHost(
		navController = navController,
		startDestination = CoreUnlockedNavSubgraph.KeySet.destination(null),
		enterTransition = {
			slideIntoContainer(
				AnimatedContentTransitionScope.SlideDirection.Start,
				animationSpec = tween()
			)
		},
		exitTransition = {
			ExitTransition.None
		},
		popEnterTransition = {
			EnterTransition.None
		},
		popExitTransition = {
			slideOutOfContainer(
				AnimatedContentTransitionScope.SlideDirection.End,
				animationSpec = tween()
			)
		}
	) {
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
				it.arguments?.getString(CoreUnlockedNavSubgraph.KeySet.seedName)!!

			DerivationCreateSubgraph(
				onBack = { navController.popBackStack() },
				onOpenCamera = { navController.navigate(CoreUnlockedNavSubgraph.camera) },
				seedName = seedName,
			)
		}
		composable(
			CoreUnlockedNavSubgraph.camera,
			enterTransition = {
				slideIntoContainer(
					AnimatedContentTransitionScope.SlideDirection.Up,
					animationSpec = tween()
				)
			},
			exitTransition = {
				slideOutOfContainer(
					AnimatedContentTransitionScope.SlideDirection.Down,
					animationSpec = tween()
				)
			},
		) {
			ScanNavSubgraph(
				onCloseCamera = {
					navController.popBackStack()
				},
				openKeySet = { seedName ->
					navController.navigate(
						CoreUnlockedNavSubgraph.KeySet.destination(
							seedNameValue = seedName
						)
					)
				}
			)
		}
		settingsFullSubgraph(
			coreNavController = navController,
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
	const val newKeySet = "core_new_keyset"
	const val recoverKeySet = "keyset_recover_flow"
	const val camera = "unlocked_camera"

	object KeySet {
		internal const val seedName = "seed_name_arg"
		private const val baseRoute = "core_keyset_details_home"
		const val route = "$baseRoute?$seedName={$seedName}" //optional
		fun destination(seedNameValue: String?): String {
			val result =
				if (seedNameValue == null) baseRoute else "$baseRoute?$seedName=${seedNameValue}"
			return result
		}
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

	object ErrorScreenGeneral {
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
	const val errorWrongDbVersionUpdate = "core_wrong_version_mismatch"

	const val airgapBreached = "core_airgap_blocker"
	const val settings = "core_settings_flow"
	const val networkHelpers = "network_helpers_path"
}
