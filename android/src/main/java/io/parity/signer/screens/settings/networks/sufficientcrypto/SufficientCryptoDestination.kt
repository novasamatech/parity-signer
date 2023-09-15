package io.parity.signer.screens.settings.networks.sufficientcrypto

import androidx.compose.runtime.remember
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavController
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavType
import androidx.navigation.compose.composable
import androidx.navigation.navArgument
import io.parity.signer.screens.settings.SettingsNavSubgraph
import kotlinx.coroutines.runBlocking


fun NavGraphBuilder.sufficientCryptoDestination(
	navController: NavController,
) {
	composable(
		route = SettingsNavSubgraph.SignNetworkSufficientCrypto.route,
		arguments = listOf(
			navArgument(SettingsNavSubgraph.SignNetworkSufficientCrypto.networkKey) {
				type = NavType.StringType
			}
		)
	) {
		val networkKey =
			it.arguments?.getString(SettingsNavSubgraph.SignNetworkSufficientCrypto.networkKey)!!
		val vm: SignSufficientCryptoViewModel = viewModel()
		val model = remember {
			runBlocking {
				vm.getNetworkModel(networkKey)!!
				//todo dmitry post error
			}
		}
		SignSufficientCryptoFull(model)
	}
	composable(
		route = SettingsNavSubgraph.SignMetadataSufficientCrypto.route,
		arguments = listOf(
			navArgument(SettingsNavSubgraph.SignMetadataSufficientCrypto.networkKey) {
				type = NavType.StringType
			},
			navArgument(SettingsNavSubgraph.SignMetadataSufficientCrypto.metadataSpecVer) {
				type = NavType.StringType
			},
		)
		) {
		val networkKey =
			it.arguments?.getString(SettingsNavSubgraph.SignMetadataSufficientCrypto.networkKey)!!
		val metadataSpecVer =
			it.arguments?.getString(SettingsNavSubgraph.SignMetadataSufficientCrypto.metadataSpecVer)!!

		val vm: SignSufficientCryptoViewModel = viewModel()
		val model = remember {
			runBlocking {
				vm.getMetadataModel(networkKey, metadataSpecVer)!!
				//todo dmitry post error
			}
		}
		SignSufficientCryptoFull(model)
	}
}
