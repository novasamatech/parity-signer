package io.parity.signer.screens.scan.transaction.transactionElements

import androidx.compose.runtime.Composable

@Composable
fun TCGenesisHash(payload: String) {
	TCNameValueTemplate(name = "Genesis hash", value = payload)
}
