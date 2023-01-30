package io.parity.signer.screens.scan

import android.widget.Toast
import androidx.activity.compose.BackHandler
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.lifecycle.viewModelScope
import androidx.lifecycle.viewmodel.compose.viewModel
import io.parity.signer.R
import io.parity.signer.bottomsheets.password.EnterPassword
import io.parity.signer.components.panels.BottomBarSingleton
import io.parity.signer.components.panels.toAction
import io.parity.signer.domain.Navigator
import io.parity.signer.screens.scan.bananasplit.BananaSplitPasswordScreen
import io.parity.signer.screens.scan.camera.ScanScreen
import io.parity.signer.screens.scan.elements.PresentableErrorModel
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

	val transactions = scanViewModel.transactions.collectAsState()
	val signature = scanViewModel.signature.collectAsState()
	val bananaSplitPassword = scanViewModel.bananaSplitPassword.collectAsState()

	val presentableError = scanViewModel.presentableError.collectAsState()
	val passwordModel = scanViewModel.passwordModel.collectAsState()
	val errorWrongPassword = scanViewModel.errorWrongPassword.collectAsState()

	val showingModals = presentableError.value != null ||
		passwordModel.value != null || errorWrongPassword.value

	val navigateToPrevious = {
		rootNavigator.navigate(BottomBarSingleton.lastUsedTab.toAction())
	}

	val backAction = {
		val wasState = scanViewModel.ifHasStateThenClear()
		if (!wasState) navigateToPrevious()
	}
	BackHandler(onBack = backAction)

	val context = LocalContext.current

	//Full screens
	val transactionsValue = transactions.value
	val bananaQrData = bananaSplitPassword.value
	if (bananaQrData != null) {
		BananaSplitPasswordScreen(
			qrData = bananaQrData,
			onClose = {
				backAction()
			},
			onSuccess = { seedName ->
				scanViewModel.clearState()
				rootNavigator.navigate(Action.SELECT_SEED, seedName)
			},
			onCustomError = { error ->
				scanViewModel.presentableError.value = PresentableErrorModel(details = error)
				scanViewModel.bananaSplitPassword.value = null
			},
			onErrorWrongPassword = {
				scanViewModel.errorWrongPassword.value = true
				scanViewModel.bananaSplitPassword.value = null
			},
			modifier = Modifier.statusBarsPadding(),
		)
	} else if (transactionsValue == null || showingModals) {

		ScanScreen(
			onClose = { navigateToPrevious() },
			performPayloads = { payloads ->
				scanViewModel.performPayload(payloads, context)
			},
			onBananaSplit = { payloads ->
				scanViewModel.bananaSplitPassword.value = payloads
			}
		)
	} else {

		TransactionsScreenFull(
			transactions = transactionsValue.transactions,
			signature = signature.value,
			modifier = Modifier.statusBarsPadding(),
			onBack = {
				backendAction(Action.GO_BACK, "", "")
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
				scanViewModel.clearState()
				rootNavigator.navigate(Action.GO_FORWARD)
			},
			onImportKeys = {
				scanViewModel.onImportKeysTap(transactionsValue, context)
			}
		)
	}
	//Bottom sheets
	presentableError.value?.let { presentableErrorValue ->
		BottomSheetWrapperRoot(onClosedAction = scanViewModel::clearState) {
			ScanErrorBottomSheet(
				error = presentableErrorValue,
				onOK = scanViewModel::clearState,
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


