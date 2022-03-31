package io.parity.signer.components.transactionCards

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Lock
import androidx.compose.runtime.Composable
import io.parity.signer.components.Identicon
import io.parity.signer.models.decode64
import io.parity.signer.ui.theme.Crypto400
import io.parity.signer.ui.theme.Text400
import io.parity.signer.ui.theme.Text600
import io.parity.signer.ui.theme.Typography
import org.json.JSONObject

@Composable
fun TCAuthor(payload: JSONObject) {
	Row {
		Identicon(payload.optString("identicon"))
		Column {
			Text(
				"From: ",
				style = MaterialTheme.typography.body1,
				color = MaterialTheme.colors.Text400
			)
			Row {
				Text(
					payload.getString("seed").decode64(),
					style = MaterialTheme.typography.body1,
					color = MaterialTheme.colors.Crypto400
				)
				Text(
					payload.getString("derivation_path").toString(),
					style = Typography.body1,
					color = MaterialTheme.colors.Crypto400
				)
				if (payload.getBoolean("has_password")) {
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
				payload.getString("base58"),
				style = MaterialTheme.typography.caption,
				color = MaterialTheme.colors.Text600
			)
		}
	}
}
