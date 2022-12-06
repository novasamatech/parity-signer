package io.parity.signer.screens.scan.transaction.transactionCards

import androidx.compose.runtime.Composable

@Composable
fun TCTipPlain(text: String) {
	TCNameValueTemplate(name = "Tip", value = text)
}
