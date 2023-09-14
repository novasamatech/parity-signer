package io.parity.signer.screens.settings.networks.signnetworkcrypto

import androidx.compose.runtime.remember
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavController
import androidx.navigation.NavGraphBuilder
import androidx.navigation.NavType
import androidx.navigation.compose.composable
import androidx.navigation.navArgument
import io.parity.signer.domain.backend.mapError
import io.parity.signer.screens.settings.SettingsNavSubgraph
import io.parity.signer.screens.settings.verifiercert.VerifierCertViewModel
import io.parity.signer.ui.mainnavigation.CoreUnlockedNavSubgraph
import kotlinx.coroutines.runBlocking


fun NavGraphBuilder.sufficientCryptoDestination(
	navController: NavController,
) {

	//todo dmitry implement
//				as SignSufficientCryptoInteractor done
//				navstate.rs:830 it's Sign sufficient crypto


//		SignSufficientCrypto(
//			screenData.f,
//			sharedViewModel::signSufficientCrypto
//		)
// end of action here calling go forward and it's in navstate.rs:427
//			todo dmitry handle password here on action
//			io/parity/signer/domain/storage/TransactionOld.kt:8 ^^


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
