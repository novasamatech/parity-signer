package io.parity.signer.screens.scan.transaction.transactionCards

import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import io.parity.signer.uniffi.MscEnumVariantName

@Composable
fun TCEnumVariantName(name: MscEnumVariantName) {
	//TODO: add docs
	Text(name.name)
}
