package io.parity.signer.ui

import androidx.lifecycle.ViewModel
import io.parity.signer.uniffi.MTransaction

/**
 * To pass shared objects from memory between different scan flow components
 */
class ScanViewModel: ViewModel() {
	var pendingTransactions: List<MTransaction> = emptyList()
}
