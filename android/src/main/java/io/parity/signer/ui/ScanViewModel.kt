package io.parity.signer.ui

import androidx.lifecycle.ViewModel
import io.parity.signer.uniffi.MTransaction
import kotlinx.coroutines.flow.MutableStateFlow

/**
 * To pass shared objects from memory between different scan flow components
 */
class ScanViewModel : ViewModel() {
	var pendingTransactions: MutableStateFlow<List<MTransaction>> =
		MutableStateFlow(emptyList())
}
