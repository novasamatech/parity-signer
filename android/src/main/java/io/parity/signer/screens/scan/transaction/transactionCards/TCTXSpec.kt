package io.parity.signer.screens.scan.transaction.transactionCards

import androidx.compose.runtime.Composable

@Composable
fun TCTXSpec(text: String) {
	TCNameValueTemplate(name = "TX version", value = text)
}
