package io.parity.signer.screens.scan.transaction.transactionCards

import androidx.compose.runtime.Composable

@Composable
fun TCPallet(text: String) {
	TCNameValueTemplate(name = "Pallet", value = text)
}
