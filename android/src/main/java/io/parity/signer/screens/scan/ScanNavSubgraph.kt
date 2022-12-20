package io.parity.signer.screens.scan

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Modifier
import androidx.lifecycle.viewmodel.compose.viewModel
import io.parity.signer.bottomsheets.password.EnterPassword
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
	val scope = rememberCoroutineScope()

	val backAction = {
		val wasState = scanViewModel.ifHasStateThenClear()
		if (!wasState) 		rootNavigator.backAction()
	}

	BackHandler(onBack = backAction)

	val transactions = scanViewModel.transactions.collectAsState()
	val signature = scanViewModel.signature.collectAsState()
	val presentableError = scanViewModel.presentableError.collectAsState()
	val passwordModel = scanViewModel.passwordModel.collectAsState()


	//Full screens

	if (transactions.value.isEmpty()) {
		ScanScreen(
			onClose = { rootNavigator.backAction() },
			performPayloads = { payloads ->
				scope.launch {
					scanViewModel.performPayload(payloads)
				}
			}
		)
	} else {
		//todo scan ios/NativeSigner/Screens/Scan/CameraView.swift:130
		Box(modifier = Modifier.statusBarsPadding()) {
			TransactionScreen(
				transactions = transactions.value,
				signature = signature.value,
				onBack = backAction,
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

	//Bottom sheets

	presentableError.value?.let { presentableError ->
		BottomSheetWrapperRoot(onClosedAction = backAction) {
			ScanErrorBottomSheet(
				presentableError,
				onClose = backAction,
			)
		}
	} ?: passwordModel.value?.let { passwordModel ->
		//todo scan ios/NativeSigner/Screens/Scan/CameraView.swift:138

		BottomSheetWrapperRoot(onClosedAction = backAction) {
			EnterPassword(
				data = passwordModel,
				proceed = {}, //todo scan
				onClose = backAction,
			)
		}
	}
}
