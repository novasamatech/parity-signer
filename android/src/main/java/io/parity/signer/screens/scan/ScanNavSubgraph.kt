package io.parity.signer.screens.scan

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Modifier
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.bottomsheets.password.EnterPassword
import io.parity.signer.bottomsheets.password.EnterPasswordModel
import io.parity.signer.models.Navigator
import io.parity.signer.screens.scan.items.ScanErrorBottomSheet
import io.parity.signer.screens.scan.transaction.TransactionScreen
import io.parity.signer.ui.BottomSheetWrapperRoot
import io.parity.signer.uniffi.Action
import kotlinx.coroutines.launch

/**
 * Navigation Subgraph with compose nav controller for those Key Set screens which are not part of general
 * Rust-controlled navigation
 */
@Composable
fun ScanNavSubgraph(
	rootNavigator: Navigator,
) {
	val scanViewModel: ScanViewModel = viewModel()
	val navController = rememberNavController()
	val scope = rememberCoroutineScope()

			BackHandler() {
				rootNavigator.backAction()
			}

	val transactions = scanViewModel.transactions.collectAsState()
	val signature = scanViewModel.signature.collectAsState()

	if (transactions.value.isEmpty()) {
		ScanScreen(
			onClose = { rootNavigator.backAction() },
			performPayloads = { payloads ->
				scope.launch {
					val transactions = scanViewModel.performPayload(payloads)
//						todo scan
//						scanViewModel.pendingTransactions.value = transactions
//						navController.navigate(ScanNavSubgraph.transaction)
				}
			}
		)
	} else {
		//todo scan ios/NativeSigner/Screens/Scan/CameraView.swift:130
		Box(modifier = Modifier.statusBarsPadding()) {
			TransactionScreen(
				transactions = scanViewModel.transactions.collectAsState().value,
				signature = scanViewModel.signature.collectAsState().value,
				onBack = {
					//was navigate(Action.GO_BACK, "", "")
					navController.navigate(ScanNavSubgraph.camera)
				},
				onFinish = {
					rootNavigator.navigate(Action.GO_FORWARD)
					scanViewModel.transactions.value = emptyList()
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
	//bottom sheets
	if ()
		composable(ScanNavSubgraph.password) {
			//todo scan ios/NativeSigner/Screens/Scan/CameraView.swift:138
			val backAction = {
				navController.navigate(ScanNavSubgraph.camera)
			}

			BackHandler(onBack = backAction)
			ScanScreen(onClose = { }, performPayloads = {})
			BottomSheetWrapperRoot(onClosedAction = {
				backAction
			}) {
				EnterPassword(
//					data = modalData.f.toEnterPasswordModel(),
//					proceed = { password ->
//						navigator.navigate(
//							Action.GO_FORWARD,
//							password
//						)
//					},
					data = EnterPasswordModel.createStub(), //todo scan
					proceed = {},
					onClose = backAction,
				)
			}
		}
		composable(ScanNavSubgraph.scanError) {
			val backAction = {
				navController.navigate(ScanNavSubgraph.camera)
			}

			BackHandler(onBack = backAction)
			ScanScreen(onClose = { }, performPayloads = {})
			BottomSheetWrapperRoot(onClosedAction = {
				backAction
			}) {
				ScanErrorBottomSheet(
"error message",//todod pass real error
					onClose = backAction,
				)
			}
		}

	}
}

private object ScanNavSubgraph {
	const val camera = "scan_camera"
	const val transaction = "scan_transaction"
	const val scanError = "scan error"
	const val password = "password"
}
