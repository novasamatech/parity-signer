package io.parity.signer.components.transactionCards

import androidx.compose.runtime.Composable

@Composable
fun TCTXSpec(text: String) {
	TCNameValueTemplate(name = "TX version", value = text)
}
