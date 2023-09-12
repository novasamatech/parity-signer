package io.parity.signer.screens.settings

import androidx.compose.foundation.layout.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.navigation.NavController
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import androidx.navigation.navArgument
import io.parity.signer.components.documents.PpScreen
import io.parity.signer.components.documents.TosScreen
import io.parity.signer.screens.settings.backup.SeedBackupIntegratedScreen
import io.parity.signer.screens.settings.general.SettingsGeneralNavSubgraph
import io.parity.signer.screens.settings.logs.logsNavigationSubgraph
import io.parity.signer.screens.settings.networks.details.NetworkDetailsSubgraph
import io.parity.signer.screens.settings.networks.list.networkListDestination
import io.parity.signer.screens.settings.verifiercert.verifierSettingsDestination

/**
 * Settings screen; General purpose stuff like legal info, networks management
 * and history should be here. This is final point in navigation:
 * all subsequent interactions should be in modals or drop-down menus
 */
@Composable
fun SettingsScreenSubgraph(
	coreNavController: NavController,
) {
// todo dmitry like
//	io/parity/signer/screens/settings/networks/list/NetworksListSubgraphOld.kt:14
	val navController = rememberNavController()
	NavHost(
		navController = navController,
		startDestination = SettingsNavSubgraph.home,
	) {

		composable(SettingsNavSubgraph.home) {
			SettingsGeneralNavSubgraph(parentNavController = navController)
		}
		composable(SettingsNavSubgraph.terms) {
			Box(modifier = Modifier.statusBarsPadding()) {
				TosScreen(onBack = {
					navController.popBackStack(SettingsNavSubgraph.home, false)
				})
			}
		}
		composable(SettingsNavSubgraph.privacyPolicy) {
			Box(modifier = Modifier.statusBarsPadding()) {
				PpScreen(onBack = {
					navController.popBackStack(SettingsNavSubgraph.home, false)
				})
			}
		}
		composable(SettingsNavSubgraph.backup) {
			SeedBackupIntegratedScreen(navController) {
				navController.popBackStack(SettingsNavSubgraph.home, false)
			}
		}
		logsNavigationSubgraph(
			navController = navController,
		)
		networkListDestination(navController)
		verifierSettingsDestination(navController)
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
				navController,
			)
		}
		composable(
			route = SettingsNavSubgraph.NetworkDetails.route,
		) {
//		SignSufficientCrypto(
//			screenData.f,
//			sharedViewModel::signSufficientCrypto
//		)
// end of action here calling go forward and it's in navstate.rs:427
//			todo dmitry handle password here on action
//			io/parity/signer/domain/storage/TransactionOld.kt:8 ^^
		}
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
	const val networkSignSufficientCrypto = "signsufficientcryptoggg"
}

