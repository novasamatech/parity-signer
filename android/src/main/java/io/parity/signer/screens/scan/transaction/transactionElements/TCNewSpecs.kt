package io.parity.signer.screens.scan.transaction.transactionElements

import androidx.compose.foundation.layout.Column
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import io.parity.signer.uniffi.NetworkSpecs

@Composable
fun TCNewSpecs(specs: NetworkSpecs) {
	Column {
		Text("NEW NETWORK")
		TCNameValueElement(name = "Network name:", value = specs.title)
		TCNameValueElement(
			name = "base58 prefix:",
			value = specs.base58prefix.toString()
		)
		TCNameValueElement(name = "decimals:", value = specs.decimals.toString())
		TCNameValueElement(name = "unit:", value = specs.unit)
		TCNameValueElement(
			name = "genesis hash:",
			value = specs.genesisHash.toString()
		)
		TCNameValueElement(name = "crypto:", value = specs.encryption.toString())
		TCNameValueElement(name = "spec name:", value = specs.name)
		TCNameValueElement(name = "logo:", value = specs.logo)
		TCNameValueElement(name = "default path", value = specs.pathId)
	}
}
