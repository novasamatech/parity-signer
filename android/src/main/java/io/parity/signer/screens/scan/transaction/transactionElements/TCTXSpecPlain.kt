package io.parity.signer.screens.scan.transaction.transactionElements

import androidx.compose.foundation.layout.Column
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import io.parity.signer.models.encodeHex
import io.parity.signer.uniffi.MscTxSpecPlain

@Composable
fun TCTXSpecPlain(specsPlain: MscTxSpecPlain) {
	Column {
		Text("Unknown network")
		TCNameValueElement(
			name = "Genesis hash",
			value = specsPlain.networkGenesisHash.toUByteArray()
				.toByteArray().encodeHex()
		)
		TCNameValueElement(name = "Version", value = specsPlain.version)
		TCNameValueElement(name = "Tx version", value = specsPlain.txVersion)
	}
}
