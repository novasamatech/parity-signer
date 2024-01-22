package io.parity.signer.screens.settings

import androidx.compose.animation.core.tween
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.ui.Modifier
import androidx.navigation.NavController
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavType
import androidx.navigation.compose.composable
import androidx.navigation.navArgument
import androidx.navigation.navigation
import io.parity.signer.components.documents.PpScreen
import io.parity.signer.components.documents.TosScreen
import io.parity.signer.screens.settings.backup.SeedBackupIntegratedScreen
import io.parity.signer.screens.settings.general.SettingsGeneralNavSubgraph
import io.parity.signer.screens.settings.logs.logsNavigationSubgraph
import io.parity.signer.screens.settings.networks.details.NetworkDetailsSubgraph
import io.parity.signer.screens.settings.networks.list.networkListDestination
import io.parity.signer.screens.settings.networks.signspecs.signSpecsDestination
import io.parity.signer.screens.settings.verifiercert.verifierSettingsDestination
import io.parity.signer.ui.mainnavigation.CoreUnlockedNavSubgraph

/**
 * Settings screen; General purpose stuff like legal info, networks management
 * and history should be here. This is final point in navigation:
 * all subsequent interactions should be in modals or drop-down menus
 */
fun NavGraphBuilder.settingsFullSubgraph(
	coreNavController: NavController,
) {
	navigation(
		route = CoreUnlockedNavSubgraph.settings,
		startDestination = SettingsNavSubgraph.home,
	) {
		composable(
			SettingsNavSubgraph.home,
			enterTransition = { fadeIn(animationSpec = tween(700)) },
			exitTransition = { fadeOut(animationSpec = tween(700)) },
		) {
			SettingsGeneralNavSubgraph(coreNavController = coreNavController)
		}
		composable(SettingsNavSubgraph.terms) {
			Box(modifier = Modifier.statusBarsPadding()) {
				TosScreen(onBack = {
					coreNavController.popBackStack(SettingsNavSubgraph.home, false)
				})
			}
		}
		composable(SettingsNavSubgraph.privacyPolicy) {
			Box(modifier = Modifier.statusBarsPadding()) {
				PpScreen(onBack = {
					coreNavController.popBackStack(SettingsNavSubgraph.home, false)
				})
			}
		}
		composable(SettingsNavSubgraph.backup) {
			SeedBackupIntegratedScreen(coreNavController) {
				coreNavController.popBackStack(SettingsNavSubgraph.home, false)
			}
		}
		logsNavigationSubgraph(
			navController = coreNavController,
		)
		networkListDestination(coreNavController)
		verifierSettingsDestination(coreNavController)
		composable(
			route = SettingsNavSubgraph.NetworkDetails.route,
			arguments = listOf(
				navArgument(SettingsNavSubgraph.NetworkDetails.networkKey) {
					type = NavType.StringType
				}
			),
		) {
			val networkKey =
				it.arguments?.getString(SettingsNavSubgraph.NetworkDetails.networkKey)!!
			NetworkDetailsSubgraph(
				networkKey,
				coreNavController,
			)
		}
		signSpecsDestination(coreNavController)
	}
}

internal object SettingsNavSubgraph {
	const val home = "settings_home"
	const val terms = "settings_terms_of_service"
	const val privacyPolicy = "settings_privacy_polcy"
	const val backup = "settings_backup"
	const val logs = "settings_logs"
	const val networkList = "settings_manage_networks"
	const val generalVerifier = "settings_general_verifier"

	object NetworkDetails {
		internal const val networkKey = "network_key"
		private const val baseRoute = "settings_network_details"
		const val route = "$baseRoute/{$networkKey}"
		fun destination(networkKey: String) = "$baseRoute/${networkKey}"
	}

	object SignNetworkSpecs {
		internal const val networkKey = "network_key"
		private const val baseRoute = "settings_network_sufficient_crypto"
		const val route = "$baseRoute/{$networkKey}"
		fun destination(networkKey: String) = "$baseRoute/${networkKey}"
	}

	object SignMetadataSpecs {
		internal const val networkKey = "network_key"
		internal const val metadataSpecVer = "spec_ver"
		private const val baseRoute = "settings_metadata_sufficient_crypto"
		const val route = "$baseRoute/{$networkKey}/{$metadataSpecVer}"
		fun destination(networkKey: String, metadataSpecVer: String) =
			"$baseRoute/${networkKey}/${metadataSpecVer}"
	}
}

