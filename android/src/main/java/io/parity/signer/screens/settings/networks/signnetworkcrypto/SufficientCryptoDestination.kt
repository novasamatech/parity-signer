package io.parity.signer.screens.settings.networks.signnetworkcrypto

import androidx.compose.runtime.remember
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavController
import androidx.navigation.NavGraphBuilder
import androidx.navigation.compose.composable
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


	composable(route = SettingsNavSubgraph.SignNetworkSufficientCrypto.route) {
//		val vm: NetworkListViewModel = viewModel()
//
//		val model = remember {
//			runBlocking {
//				vm.getNetworkList().mapError()!!
//				//todo dmitry post error
//			}
//		}

		//		SignSufficientCrypto(
//			screenData.f,
//			sharedViewModel::signSufficientCrypto
//		)
	}
	composable(SettingsNavSubgraph.SignMetadataSufficientCrypto.route) {
		val vm: VerifierCertViewModel = viewModel()

		val model = remember {
			runBlocking {
				vm.getVerifierCertModel().mapError()!!
				//todo dmitry post error
			}
		}
		VerifierScreenFull(
			verifierDetails = model,
			wipe = {
				vm.wipeWithGeneralCertificate {
					navController.navigate(
						CoreUnlockedNavSubgraph.keySetList
					)
				}
			},
			onBack = navController::popBackStack,
		)
	}
}
