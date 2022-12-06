package io.parity.signer.screens.scan.transaction.transactionElements

import androidx.compose.runtime.Composable

@Composable
fun TCNonce(text: String) {
	TCNameValueTemplate(name = "Nonce", value = text)
}
