package io.parity.signer.components.transactionCards

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import io.parity.signer.components.Identicon
import io.parity.signer.uniffi.MscId

@Composable
fun TCID(id: MscId) {
	Row {
		Identicon(identicon = id.identicon)
		Column {
			Text(id.base58)
		}
	}
}
