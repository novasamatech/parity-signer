package io.parity.signer.components.transactionCards

import androidx.compose.runtime.Composable

@Composable
fun TCGenesisHash(payload: String) {
	TCNameValueTemplate(name = "Genesis hash", value = payload)
}
