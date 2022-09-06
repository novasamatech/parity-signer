package io.parity.signer.components.transactionCards

import androidx.compose.runtime.Composable

@Composable
fun TCVarName(text: String) {
	TCNameValueTemplate(name = "", value = text)
}
