package io.parity.signer.screens.scan.transaction.transactionElements

import androidx.compose.runtime.Composable

@Composable
fun TCBitVec(bitVec: String) {
	TCNameValueTemplate(name = bitVec, value = bitVec)
}
