package io.parity.signer.components.transactionCards

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import io.parity.signer.components.IdentIcon
import io.parity.signer.components.toImageContent
import io.parity.signer.uniffi.MscId

@Composable
fun TCID(id: MscId) {
	Row {
		IdentIcon(identicon = id.identicon.toImageContent())
		Column {
			Text(id.base58)
		}
	}
}
