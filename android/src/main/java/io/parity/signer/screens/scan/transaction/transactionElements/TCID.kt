package io.parity.signer.screens.scan.transaction.transactionElements

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import io.parity.signer.components.IdentIcon
import io.parity.signer.uniffi.MscId

@Composable
fun TCID(id: MscId) {
	Row {
		IdentIcon(identicon = id.identicon)
		Column {
			Text(id.base58)
		}
	}
}
