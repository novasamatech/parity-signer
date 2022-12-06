package io.parity.signer.screens.scan.transaction.transactionElements

import androidx.compose.runtime.Composable

@Composable
fun TCIdentityField(text: String) {
	TCNameValueTemplate(name = "IdentityField", value = text)
}
