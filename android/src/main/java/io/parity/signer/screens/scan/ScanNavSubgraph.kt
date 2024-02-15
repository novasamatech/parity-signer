package io.parity.signer.screens.scan

import android.widget.Toast
import androidx.activity.compose.BackHandler
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.MutableState
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewModelScope
import androidx.lifecycle.viewmodel.compose.viewModel
import io.parity.signer.R
import io.parity.signer.bottomsheets.password.EnterPassword
import io.parity.signer.domain.Callback
import io.parity.signer.domain.FakeNavigator
import io.parity.signer.screens.scan.addnetwork.AddedNetworkSheetsSubgraph
import io.parity.signer.screens.scan.bananasplitrestore.BananaSplitSubgraph
import io.parity.signer.screens.scan.camera.ScanScreen
import io.parity.signer.screens.scan.elements.WrongPasswordBottomSheet
import io.parity.signer.screens.scan.errors.LocalErrorBottomSheet
import io.parity.signer.screens.scan.errors.LocalErrorSheetModel
import io.parity.signer.screens.scan.transaction.TransactionPreviewType
import io.parity.signer.screens.scan.transaction.TransactionsScreenFull
import io.parity.signer.screens.scan.transaction.dynamicderivations.AddDynamicDerivationScreenFull
import io.parity.signer.screens.scan.transaction.previewType
import io.parity.signer.ui.BottomSheetWrapperRoot
import io.parity.signer.uniffi.Action
import kotlinx.coroutines.launch

/**
 * Navigation Subgraph with compose nav controller for those Key Set screens which are not part of general
 * Rust-controlled navigation
 */
@Composable
fun ScanNavSubgraph(
	onCloseCamera: Callback,
	openKeySet:(seedName: String) -> Unit,
) {
	val scanViewModel: ScanViewModel = viewModel()

	val transactions = scanViewModel.transactions.collectAsStateWithLifecycle()
	val signature = scanViewModel.signature.collectAsStateWithLifecycle()
	val bananaSplitPassword =
		scanViewModel.bananaSplitPassword.collectAsStateWithLifecycle()
	val dynamicDerivations =
		scanViewModel.dynamicDerivations.collectAsStateWithLifecycle()

	val transactionError =
		scanViewModel.transactionError.collectAsStateWithLifecycle()
	val passwordModel = scanViewModel.passwordModel.collectAsStateWithLifecycle()
	val errorWrongPassword =
		scanViewModel.errorWrongPassword.collectAsStateWithLifecycle()

	val addedNetworkName: MutableState<String?> =
		remember { mutableStateOf(null) }

	val showingModals = transactionError.value != null ||
		passwordModel.value != null || errorWrongPassword.value

	val backAction = {
		val wasState = scanViewModel.ifHasStateThenClear()
		if (!wasState) onCloseCamera()
	}
	BackHandler(onBack = backAction)

	val context = LocalContext.current

	//Full screens
	val transactionsValue = transactions.value
	val bananaQrData = bananaSplitPassword.value
	val dynamicDerivationsData = dynamicDerivations.value
	if (bananaQrData != null) {
		BananaSplitSubgraph(
			qrData = bananaQrData,
			onClose = {
				backAction()
			},
			onSuccess = { seedName ->
				Toast.makeText(
					context,
					context.getString(
						R.string.key_set_has_been_recovered_toast,
						seedName
					),
					Toast.LENGTH_LONG
				).show()
				scanViewModel.clearState()
				openKeySet(seedName)
			},
			onCustomError = { error ->
				scanViewModel.transactionError.value =
					LocalErrorSheetModel(context = context, details = error)
				scanViewModel.bananaSplitPassword.value = null
			},
			onErrorWrongPassword = {
				scanViewModel.errorWrongPassword.value = true
				scanViewModel.bananaSplitPassword.value = null
			},
		)
	} else if (dynamicDerivationsData != null) {
		AddDynamicDerivationScreenFull(
			model = dynamicDerivationsData,
			onClose = scanViewModel::clearState,
		)
	} else if (transactionsValue == null || showingModals) {

		ScanScreen(
			onClose = onCloseCamera,
			performPayloads = { payloads ->
				scanViewModel.performTransactionPayload(payloads, context)
			},
			onBananaSplit = { payloads ->
				scanViewModel.bananaSplitPassword.value = payloads
			},
			onDynamicDerivations = { payload ->
				scanViewModel.performDynamicDerivationPayload(payload, context)
			},
			onDynamicDerivationsTransactions = { payload ->
				scanViewModel.performDynamicDerivationTransaction(payload, context)
			},
		)
	} else {

		TransactionsScreenFull(
			transactions = transactionsValue.transactions,
			signature = signature.value,
			modifier = Modifier.statusBarsPadding(),
			onBack = {
				FakeNavigator().backAction()
				scanViewModel.clearState()
			},
			onApprove = {
				when (val previewType =
					transactions.value?.transactions?.previewType) {
					is TransactionPreviewType.AddNetwork -> {
						Toast.makeText(
							context,
							context.getString(
								R.string.toast_network_added,
								previewType.network
							),
							Toast.LENGTH_LONG
						).show()
						addedNetworkName.value = previewType.network
					}

					is TransactionPreviewType.Metadata -> {
						Toast.makeText(
							context,
							context.getString(
								R.string.toast_metadata_added,
								previewType.network,
								previewType.version
							),
							Toast.LENGTH_LONG
						).show()
					}

					else -> {
						//nothing
					}
				}
				//finally clear transaction state and stay in scan screen
				scanViewModel.clearState()
				val fakeNavigator = FakeNavigator()
				fakeNavigator.navigate(Action.GO_FORWARD)
				fakeNavigator.navigate(Action.START)
				fakeNavigator.navigate(Action.NAVBAR_SCAN)
			},
			onImportKeys = {
				scanViewModel.onImportKeysTap(transactionsValue, context)
			}
		)
	}
	//Bottom sheets
	transactionError.value?.let { presentableErrorValue ->
		BottomSheetWrapperRoot(onClosedAction = scanViewModel::clearState) {
			LocalErrorBottomSheet(
				error = presentableErrorValue,
				onOk = scanViewModel::clearState,
			)
		}
	} ?: passwordModel.value?.let { passwordModelValue ->
		BottomSheetWrapperRoot(onClosedAction = {
			scanViewModel.resetRustModalToNewScan()
			scanViewModel.clearState()
		}) {
			EnterPassword(
				data = passwordModelValue,
				proceed = { password ->
					scanViewModel.viewModelScope.launch {
						scanViewModel.handlePasswordEntered(password)
					}
				},
				onClose = {
					scanViewModel.resetRustModalToNewScan()
					scanViewModel.clearState()
				},
			)
		}
	} ?: addedNetworkName.value?.let { addedNetwork ->
		AddedNetworkSheetsSubgraph(
			networkNameAdded = addedNetwork,
			onClose = {
				addedNetworkName.value = null
			}
		)
	} ?: if (errorWrongPassword.value) {
		BottomSheetWrapperRoot(onClosedAction = scanViewModel::clearState) {
			WrongPasswordBottomSheet(
				onOk = scanViewModel::clearState
			)
		}
	} else {
		//no bottom sheet
	}
}


