package io.parity.signer.components.transactionCards

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import io.parity.signer.components.IdentIcon
import io.parity.signer.components.toImageContent
import io.parity.signer.ui.theme.Text400
import io.parity.signer.ui.theme.Text600
import io.parity.signer.uniffi.MscId

@Composable
fun TCAuthorPlain(author: MscId) {
	Row {
		IdentIcon(author.identicon.toImageContent())
		Column {
			Text(
				"From: ",
				style = MaterialTheme.typography.body1,
				color = MaterialTheme.colors.Text400
			)
			Text(
				author.base58,
				style = MaterialTheme.typography.body1,
				color = MaterialTheme.colors.Text600
			)
		}
	}
}
