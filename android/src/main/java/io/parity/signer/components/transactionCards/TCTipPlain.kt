package io.parity.signer.components.transactionCards

import androidx.compose.runtime.Composable

@Composable
fun TCTipPlain(text: String) {
	TCNameValueTemplate(name = "Tip", value = text)
}
