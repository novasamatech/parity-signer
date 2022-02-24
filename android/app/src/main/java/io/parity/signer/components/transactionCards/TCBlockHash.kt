package io.parity.signer.components.transactionCards

import androidx.compose.runtime.Composable

@Composable
fun TCBlockHash(text: String) {
	TCNameValueTemplate(name = "Block hash", value = text)
}
