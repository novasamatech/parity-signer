package io.parity.signer.screens.scan

import android.util.Log
import androidx.lifecycle.ViewModel
import io.parity.signer.backend.UniffiResult
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.uniffi.*
import kotlinx.coroutines.flow.MutableStateFlow

/**
 * Shared ViewModel for all Scan flow components, not only camera related.
 */
class ScanViewModel : ViewModel() {
	private val uniffiInteractor = ServiceLocator.backendLocator.uniffiInteractor

	var pendingTransactions: MutableStateFlow<List<MTransaction>> =
		MutableStateFlow(emptyList())
	var signature: MutableStateFlow<MSignatureReady?> =
		MutableStateFlow(null)


	suspend fun performPayloads(payloads: Set<String>): List<MTransaction> {
		val allResults = payloads.map { payload ->
			uniffiInteractor.navigate(Action.TRANSACTION_FETCHED, payload)
		}
		//todo handle error cases and show ot user?
		allResults.filterIsInstance<UniffiResult.Error<Any>>().forEach { error ->
			Log.e("scanVM","Camera scan: " + "transaction parsing failed, ${error.error.message}")
		}
		return allResults.filterIsInstance<UniffiResult.Success<ActionResult>>()
			.mapNotNull { (it.result.screenData as? ScreenData.Transaction)?.f }.flatten()
	}
}
