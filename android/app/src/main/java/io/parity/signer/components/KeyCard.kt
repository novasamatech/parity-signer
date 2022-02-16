package io.parity.signer.components

import androidx.compose.foundation.layout.*
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Lock
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.models.abbreviateString
import io.parity.signer.ui.theme.Crypto400
import io.parity.signer.ui.theme.CryptoTypography
import io.parity.signer.ui.theme.Text400
import io.parity.signer.ui.theme.Text600
import org.json.JSONObject

/**
 * A card to show key info; only visual things.
 * TODO: paint root keys in scary colors
 */
@Composable
fun KeyCard(identity: JSONObject) {
	Row(
		modifier = Modifier
			.padding(8.dp)
	) {
		Identicon(identity.optString("identicon"))
		Spacer(modifier = Modifier.width(10.dp))
		Column {
			Row {
				Text(
					identity.optString("seed_name"),
					color = MaterialTheme.colors.Text600,
					style = MaterialTheme.typography.subtitle1
				)
				Text(
					identity.optString("path", identity.optString("derivation_path")),
					color = MaterialTheme.colors.Crypto400,
					style = CryptoTypography.body2
				)
				if (identity.optBoolean("has_pwd", false)) {
					Text(
						"///",
						color = MaterialTheme.colors.Crypto400,
						style = CryptoTypography.body2
					)
					Icon(
						Icons.Default.Lock,
						contentDescription = "Locked account",
						tint = MaterialTheme.colors.Crypto400
					)
				}
			}
			Text(
				identity.optString("base58").abbreviateString(8),
				color = MaterialTheme.colors.Text400,
				style = CryptoTypography.body1
			)
		}
	}
}

