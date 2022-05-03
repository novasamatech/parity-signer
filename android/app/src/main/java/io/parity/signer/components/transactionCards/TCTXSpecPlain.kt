package io.parity.signer.components.transactionCards

import androidx.compose.foundation.layout.Column
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import io.parity.signer.uniffi.MscTxSpecPlain
import org.json.JSONObject

@Composable
fun TCTXSpecPlain(specsPlain: MscTxSpecPlain) {
	Column {
		Text("Unknown network")
		TCNameValueTemplate(
			name = "Genesis hash",
			value = specsPlain.networkGenesisHash
		)
		TCNameValueTemplate(name = "Version", value = specsPlain.version)
		TCNameValueTemplate(name = "Tx version", value = specsPlain.txVersion)
	}
}
