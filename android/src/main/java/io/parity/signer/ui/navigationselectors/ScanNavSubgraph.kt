package io.parity.signer.ui.navigationselectors

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Modifier
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.models.Navigator
import io.parity.signer.models.SignerDataModel
import io.parity.signer.screens.scan.ScanScreen
import io.parity.signer.screens.scan.ScanViewModel
import io.parity.signer.screens.scan.old.SignatureReady
import io.parity.signer.screens.scan.transaction.TransactionPreviewEdited
import io.parity.signer.ui.BottomSheetWrapperRoot
import io.parity.signer.uniffi.Action

/**
 * Navigation Subgraph with compose nav controller for those Key Set screens which are not part of general
 * Rust-controlled navigation
 */
@Composable
fun ScanNavSubgraph(
	signerDataModel: SignerDataModel,
	rootNavigator: Navigator,
) {
	val scanViewModel: ScanViewModel = viewModel()
	val navController = rememberNavController()
	NavHost(
		navController = navController,
		startDestination = ScanNavSubgraph.camera,
	) {

		composable(ScanNavSubgraph.camera) {
			BackHandler() {
				rootNavigator.backAction()
			}
			ScanScreen(
				onClose = { rootNavigator.backAction() },
				onNavigateToTransaction = { transactions ->
					scanViewModel.pendingTransactions.value = transactions
					navController.navigate(ScanNavSubgraph.transaction)
				}
			)
		}
		composable(ScanNavSubgraph.transaction) {
			val transactions = scanViewModel.pendingTransactions.collectAsState()

			Box(modifier = Modifier.statusBarsPadding()) {
				TransactionPreviewEdited(
					transactions = transactions.value,
					signerDataModel = signerDataModel,
					onBack = {
						//was navigate(Action.GO_BACK, "", "")
						navController.navigate(ScanNavSubgraph.camera)
					},
					onSigReady = { signature ->
						scanViewModel.signature.value = signature
						navController.navigate(ScanNavSubgraph.signatureReady)
					},
					onFinish = {
						rootNavigator.navigate(Action.GO_FORWARD)
						scanViewModel.pendingTransactions.value = emptyList()
						// todo dmitry handle subsequent modals
//						rust/navigator/src/navstate.rs:396
//						val navResult = uniffiinteractor.ProcessBatchTransactions(some_all) and handle
						//Modal::EnterPassword
						//Modal::SignatureReady(a);
						//Screen::Transaction( can come with updated checksum
						//success will clear to log
						// alert error
					},
				)
			}
		}
		composable(ScanNavSubgraph.signatureReady) {
			BackHandler() {
				navController.navigate(ScanNavSubgraph.camera)
			}
			BottomSheetWrapperRoot(onClosedAction = {
				navController.navigate(ScanNavSubgraph.camera)
			}) {
				SignatureReady(
					scanViewModel.signature.collectAsState().value!!,
					signerDataModel = signerDataModel
				)
			}
		}
	}
}

private object ScanNavSubgraph {
	const val camera = "scan_camera"
	const val transaction = "scan_transaction"
	const val signatureReady = "signature ready"
}
