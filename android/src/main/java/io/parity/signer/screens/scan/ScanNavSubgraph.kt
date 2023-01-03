package io.parity.signer.screens.scan

import android.widget.Toast
import androidx.activity.compose.BackHandler
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.currentCompositionLocalContext
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.stringResource
import androidx.lifecycle.viewmodel.compose.viewModel
import io.parity.signer.R
import io.parity.signer.bottomsheets.password.EnterPassword
import io.parity.signer.models.Navigator
import io.parity.signer.screens.scan.elements.ScanErrorBottomSheet
import io.parity.signer.screens.scan.elements.WrongPasswordBottomSheet
import io.parity.signer.screens.scan.transaction.TransactionPreviewType
import io.parity.signer.screens.scan.transaction.TransactionsScreenFull
import io.parity.signer.screens.scan.transaction.previewType
import io.parity.signer.ui.BottomSheetWrapperRoot
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.backendAction
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

	val transactions = scanViewModel.transactions.collectAsState()
	val signature = scanViewModel.signature.collectAsState()
	val presentableError = scanViewModel.presentableError.collectAsState()
	val passwordModel = scanViewModel.passwordModel.collectAsState()
	val errorWrongPassword = scanViewModel.errorWrongPassword.collectAsState()

	val showingModals =
		presentableError.value != null || passwordModel.value != null || errorWrongPassword.value

	val backAction = {
		val wasState = scanViewModel.ifHasStateThenClear()
		if (!wasState) rootNavigator.backAction()
	}
	BackHandler(onBack = backAction)

	val context = LocalContext.current

	//Full screens
	val transactionsValue = transactions.value
	if (transactionsValue == null || showingModals) {
		ScanScreen(
			onClose = { rootNavigator.backAction() },
			performPayloads = { payloads ->
				scope.launch {
					scanViewModel.performPayload(payloads)
				}
			}
		)
	} else {
		//ios/NativeSigner/Screens/Scan/CameraView.swift:130
		TransactionsScreenFull(
			transactions = transactionsValue.transactions,
			title = transactionsValue.title,
			signature = signature.value,
			modifier = Modifier.statusBarsPadding(),
			onBack = {
				backendAction(Action.GO_BACK, "", "")
				scanViewModel.clearTransactionState()
			},
			onFinish = {
				when (val previewType = transactions.value?.transactions?.firstOrNull()?.previewType) {
					is TransactionPreviewType.AddNetwork -> {
						Toast.makeText(context, context.getString(R.string.toast_network_added, previewType.network), Toast.LENGTH_LONG).show()
					}
					is TransactionPreviewType.Metadata -> {
						Toast.makeText(context, context.getString(R.string.toast_metadata_added, previewType.network, previewType.version), Toast.LENGTH_LONG).show()
					}
					else -> {
						//nothing
					}
				}
				scanViewModel.approveTransactionAction(context)
			},
		)
	}

	//Bottom sheets
	presentableError.value?.let { presentableErrorValue ->
		BottomSheetWrapperRoot(onClosedAction = scanViewModel::clearTransactionState) {
			ScanErrorBottomSheet(
				presentableErrorValue,
				onOK = scanViewModel::clearTransactionState,
			)
		}
	} ?: passwordModel.value?.let { passwordModelValue ->
		//ios/NativeSigner/Screens/Scan/CameraView.swift:138
		BottomSheetWrapperRoot(onClosedAction = scanViewModel::clearTransactionState) {
			EnterPassword(
				data = passwordModelValue,
				proceed = { password ->
					scope.launch {
						scanViewModel.handlePasswordEntered(password)
					}
				},
				onClose = scanViewModel::clearTransactionState,
			)
		}
	} ?: if (errorWrongPassword.value) {
		BottomSheetWrapperRoot(onClosedAction = scanViewModel::clearTransactionState) {
			WrongPasswordBottomSheet(
				onOk = backAction
			)
		}
	} else {
		//no bottom sheet
	}
}
