package io.parity.signer.components.transactionCards

import androidx.compose.runtime.Composable

@Composable
fun TCBitVec(bitVec: String) {
	TCNameValueTemplate(name = bitVec, value = bitVec)
}
