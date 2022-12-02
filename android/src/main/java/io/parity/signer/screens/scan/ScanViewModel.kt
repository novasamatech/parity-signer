package io.parity.signer.screens.scan

import androidx.lifecycle.ViewModel
import io.parity.signer.uniffi.MTransaction
import kotlinx.coroutines.flow.MutableStateFlow

/**
 * Shared ViewModel for all Scan flow components, not only camera related.
 */
class ScanViewModel : ViewModel() {
	var pendingTransactions: MutableStateFlow<List<MTransaction>> =
		MutableStateFlow(emptyList())

	
}
