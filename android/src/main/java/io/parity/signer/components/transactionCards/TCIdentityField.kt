package io.parity.signer.components.transactionCards

import androidx.compose.runtime.Composable

@Composable
fun TCIdentityField(text: String) {
	TCNameValueTemplate(name = "IdentityField", value = text)
}
