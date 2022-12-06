package io.parity.signer.screens.scan.transaction.transactionElements

import androidx.compose.runtime.Composable

@Composable
fun TCNetworkName(text: String) {
	TCNameValueTemplate(name = "Network name", value = text)
}
