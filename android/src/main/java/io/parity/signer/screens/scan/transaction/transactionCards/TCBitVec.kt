package io.parity.signer.screens.scan.transaction.transactionCards

import androidx.compose.runtime.Composable

@Composable
fun TCBitVec(bitVec: String) {
	TCNameValueTemplate(name = bitVec, value = bitVec)
}
