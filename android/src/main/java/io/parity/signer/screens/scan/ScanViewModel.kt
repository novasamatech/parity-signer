package io.parity.signer.screens.scan

import android.util.Log
import androidx.lifecycle.ViewModel
import io.parity.signer.backend.UniffiResult
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.models.isDisplayingErrorOnly
import io.parity.signer.uniffi.*
import kotlinx.coroutines.flow.MutableStateFlow


private const val TAG = "ScanViewModelTag"

/**
 * Shared ViewModel for all Scan flow components, not only camera related.
 */
class ScanViewModel : ViewModel() {
	private val uniffiInteractor = ServiceLocator.backendLocator.uniffiInteractor

	var pendingTransactions: MutableStateFlow<List<MTransaction>> =
		MutableStateFlow(emptyList())
	var signature: MutableStateFlow<MSignatureReady?> =
		MutableStateFlow(null)

	private val transactionInProgress = MutableStateFlow<Boolean>(false)


	suspend fun performPayloads(payloads: Set<String>): List<MTransaction> {//todo remove?
		val allResults = payloads.map { payload ->
			uniffiInteractor.navigate(Action.TRANSACTION_FETCHED, payload)
		}
		//todo handle error cases and show ot user?
		allResults.filterIsInstance<UniffiResult.Error<Any>>().forEach { error ->
			Log.e(
				TAG,
				"Camera scan: transaction parsing failed, ${error.error.message}"
			)
		}
		return allResults.filterIsInstance<UniffiResult.Success<ActionResult>>()
			.mapNotNull { (it.result.screenData as? ScreenData.Transaction)?.f }
			.flatten()
	}

	suspend fun performPayload(payload: String) {
		if (transactionInProgress.value) {
			Log.e(TAG, "started transaction while it was in progress, ignoring")
			return
		}
		transactionInProgress.value = true

		val navigateResponse =
			uniffiInteractor.navigate(Action.TRANSACTION_FETCHED, payload)
		val screenData =
			(navigateResponse as? UniffiResult.Success)?.result?.screenData
		val transaction = (screenData as? ScreenData.Transaction)?.f
			?: run {
				Log.e(
					TAG, "Error in getting transaction from qr payload, " +
						"screenData is $screenData, navigation resp is $navigateResponse"
				)
				return
			}

		// Handle transactions with just error payload
		if (transaction.all { it.isDisplayingErrorOnly() }) {
			//todo dmitry ios code see how to show, why separately?
			//         presentableError = .transactionSigningError(
			//                    message: transactions
			//                        .reduce("") { $0 + $1.transactionIssues() + ($1 == transactions.last ? "\n" : "") }
			//                )
			//                navigation.performFake(navigation: .init(action: .goBack))
			//                isPresentingError = true
			//                return
		}


	}

	fun clearTransactionState() {
		pendingTransactions.value = emptyList()
		signature.value = null
		transactionInProgress.value = false
	}

}
