package io.parity.signer.components.transactionCards

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Lock
import androidx.compose.runtime.Composable
import io.parity.signer.components.IdentIcon
import io.parity.signer.components.toImageContent
import io.parity.signer.ui.theme.Crypto400
import io.parity.signer.ui.theme.Text400
import io.parity.signer.ui.theme.Text600
import io.parity.signer.ui.theme.Typography
import io.parity.signer.uniffi.MAddressCard

@Composable
fun TCAuthor(author: MAddressCard) {
	Row {
		IdentIcon(author.address.identicon.toImageContent())
		Column {
			Text(
				"From: ",
				style = MaterialTheme.typography.body1,
				color = MaterialTheme.colors.Text400
			)
			Row {
				Text(
					author.address.seedName,
					style = MaterialTheme.typography.body1,
					color = MaterialTheme.colors.Crypto400
				)
				Text(
					author.address.path,
					style = Typography.body1,
					color = MaterialTheme.colors.Crypto400
				)
				if (author.address.hasPwd) {
					Text(
						"///",
						style = MaterialTheme.typography.body1,
						color = MaterialTheme.colors.Crypto400
					)
					Icon(
						Icons.Default.Lock,
						contentDescription = "Password protected account",
						tint = MaterialTheme.colors.Crypto400
					)
				}
			}
			Text(
				author.base58,
				style = MaterialTheme.typography.caption,
				color = MaterialTheme.colors.Text600
			)
		}
	}
}
