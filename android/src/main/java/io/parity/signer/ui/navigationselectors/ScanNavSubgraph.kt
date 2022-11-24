package io.parity.signer.ui.navigationselectors

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.models.Navigator
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.signTransaction
import io.parity.signer.screens.scan.ScanScreen
import io.parity.signer.screens.scan.transaction.TransactionPreviewEdited
import io.parity.signer.ui.ScanViewModel
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.MTransaction

/**
 * Navigation Subgraph with compose nav controller for those Key Set screens which are not part of general
 * Rust-controlled navigation
 */
@Composable
fun ScanNavSubgraph(
	signerDataModel: SignerDataModel,
	rootNavigator: Navigator,
) {
	val scanViewModel : ScanViewModel = viewModel()
	val navController = rememberNavController()
	NavHost(
		navController = navController,
		startDestination = ScanNavSubgraph.camera,
	) {

		composable(ScanNavSubgraph.camera) {
			ScanScreen(
				onClose = { rootNavigator.backAction() },
				onNavigateToTransaction = { transaction ->
					scanViewModel.pendingTransactions = transaction
					navController.navigate(ScanNavSubgraph.transaction)
				}
			)
		}
		composable(ScanNavSubgraph.transaction) {
			Box(modifier = Modifier.statusBarsPadding()) {
				TransactionPreviewEdited(
					transaction = scanViewModel.pendingTransactions.first(), //todo multisign support missing yet
					onBack = {
						//was navigate(Action.GO_BACK, "", "")
						navController.navigate(ScanNavSubgraph.camera)
					},
					onFinish = {
						rootNavigator.navigate(Action.GO_FORWARD)
						scanViewModel.pendingTransactions = emptyList()
						// todo multisign handle subsequent modals
//						rust/navigator/src/navstate.rs:396
//						val navResult = uniffiinteractor.ProcessBatchTransactions(some_all) and handle
							//Modal::EnterPassword
						//Modal::SignatureReady(a);
						//Screen::Transaction( can come with updated checksum
						//success will clear to log
										 // alert error
					},
					signTransaction = signerDataModel::signTransaction,
				)
			}
		}
	}
}

private object ScanNavSubgraph {
	const val camera = "scan_camera"
	const val transaction = "scan_transaction"
}
