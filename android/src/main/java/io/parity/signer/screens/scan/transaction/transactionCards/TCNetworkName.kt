package io.parity.signer.screens.scan.transaction.transactionCards

import androidx.compose.runtime.Composable

@Composable
fun TCNetworkName(text: String) {
	TCNameValueTemplate(name = "Network name", value = text)
}
