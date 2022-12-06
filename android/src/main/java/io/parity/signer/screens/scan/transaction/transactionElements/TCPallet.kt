package io.parity.signer.screens.scan.transaction.transactionElements

import androidx.compose.runtime.Composable

@Composable
fun TCPallet(text: String) {
	TCNameValueTemplate(name = "Pallet", value = text)
}
