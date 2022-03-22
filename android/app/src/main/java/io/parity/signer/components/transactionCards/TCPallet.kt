package io.parity.signer.components.transactionCards

import androidx.compose.runtime.Composable

@Composable
fun TCPallet(text: String) {
	TCNameValueTemplate(name = "Pallet", value = text)
}
