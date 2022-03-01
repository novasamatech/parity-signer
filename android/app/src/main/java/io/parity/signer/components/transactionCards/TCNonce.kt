package io.parity.signer.components.transactionCards

import androidx.compose.runtime.Composable

@Composable
fun TCNonce(text: String) {
	TCNameValueTemplate(name = "Nonce", value = text)
}
