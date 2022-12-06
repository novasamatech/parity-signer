package io.parity.signer.screens.scan.transaction.transactionElements

import androidx.compose.runtime.Composable

@Composable
fun TCBlockHash(text: String) {
	TCNameValueTemplate(name = "Block hash", value = text)
}
