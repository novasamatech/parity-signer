package io.parity.signer.screens.scan.transaction.transactionElements

import androidx.compose.runtime.Composable

@Composable
fun TCTXSpec(text: String) {
	TCNameValueTemplate(name = "TX version", value = text)
}
