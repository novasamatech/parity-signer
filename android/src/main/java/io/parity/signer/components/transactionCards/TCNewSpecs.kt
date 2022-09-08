package io.parity.signer.components.transactionCards

import androidx.compose.foundation.layout.Column
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import io.parity.signer.uniffi.NetworkSpecsToSend

@Composable
fun TCNewSpecs(specs: NetworkSpecsToSend) {
	Column {
		Text("NEW NETWORK")
		TCNameValueTemplate(name = "Network name:", value = specs.title)
		TCNameValueTemplate(
			name = "base58 prefix:",
			value = specs.base58prefix.toString()
		)
		TCNameValueTemplate(name = "decimals:", value = specs.decimals.toString())
		TCNameValueTemplate(name = "unit:", value = specs.unit)
		TCNameValueTemplate(
			name = "genesis hash:",
			value = specs.genesisHash.toString()
		)
		TCNameValueTemplate(name = "crypto:", value = specs.encryption.toString())
		TCNameValueTemplate(name = "spec name:", value = specs.name)
		TCNameValueTemplate(name = "logo:", value = specs.logo)
		TCNameValueTemplate(name = "default path", value = specs.pathId)
	}
}
