package io.parity.signer.screens.settings

import androidx.compose.foundation.layout.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.ui.Modifier
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.components.documents.PpScreen
import io.parity.signer.components.documents.TosScreen
import io.parity.signer.domain.Callback
import io.parity.signer.domain.Navigator
import io.parity.signer.domain.NetworkState
import io.parity.signer.screens.settings.backup.SeedBackupIntegratedScreen
import io.parity.signer.screens.settings.general.SettingsGeneralNavSubgraph
import io.parity.signer.screens.settings.logs.logsNavigationSubgraph
import io.parity.signer.screens.settings.networks.list.NetworksListSubgraph
import io.parity.signer.screens.settings.networks.list.networkListDestination

/**
 * Settings screen; General purpose stuff like legal info, networks management
 * and history should be here. This is final point in navigation:
 * all subsequent interactions should be in modals or drop-down menus
 */
@Composable
fun SettingsScreenSubgraph(
	rootNavigator: Navigator,
	isStrongBoxProtected: Boolean,
	appVersion: String,
	wipeToFactory: Callback,
	networkState: State<NetworkState?>
) {
// todo dmitry like
//	io/parity/signer/screens/settings/networks/list/NetworksListSubgraphOld.kt:14
	val navController = rememberNavController()
	NavHost(
		navController = navController,
		startDestination = SettingsScreenSubgraph.home,
	) {

		composable(SettingsScreenSubgraph.home) {
			SettingsGeneralNavSubgraph(
				rootNavigator = rootNavigator,
				parentNavController = navController,
				isStrongBoxProtected = isStrongBoxProtected,
				appVersion = appVersion,
				wipeToFactory = wipeToFactory,
				networkState = networkState,
			)
		}
		composable(SettingsScreenSubgraph.terms) {
			Box(modifier = Modifier.statusBarsPadding()) {
				TosScreen(onBack = {
					navController.popBackStack(SettingsScreenSubgraph.home, false)
				})
			}
		}
		composable(SettingsScreenSubgraph.privacyPolicy) {
			Box(modifier = Modifier.statusBarsPadding()) {
				PpScreen(onBack = {
					navController.popBackStack(SettingsScreenSubgraph.home, false)
				})
			}
		}
		composable(SettingsScreenSubgraph.backup) {
			SeedBackupIntegratedScreen(rootNavigator) {
				navController.popBackStack(SettingsScreenSubgraph.home, false)
			}
		}
		logsNavigationSubgraph(
			SettingsScreenSubgraph.logs,
			rootNavigator,
			navController
		)
		networkListDestination(navController)

		composable(SettingsScreenSubgraph.generalVerifier) {
			//todo dmitry implement
//			is ScreenData.VVerifier -> VerifierScreenFull(
//			screenData.f.toVerifierDetailsModels(),
//			sharedViewModel::wipeToJailbreak,
//			rootNavigator,
//			)
		}
		composable(SettingsScreenSubgraph.NetworkDetails.route) {
			//todo dmitry single network subgraph
//		NetworkDetailsSubgraph(
//			screenData.f.toNetworkDetailsModel(),
//			rootNavigator,
//		)
		}
	}
}

internal object SettingsScreenSubgraph {
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
		fun destination(seedName: String) = "$baseRoute/${networkKey}"
	}
}

