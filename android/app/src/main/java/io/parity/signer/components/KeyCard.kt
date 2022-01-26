package io.parity.signer.components

import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Lock
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.scale
import androidx.compose.ui.unit.dp
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.abbreviateString
import io.parity.signer.models.intoImageBitmap
import io.parity.signer.ui.theme.Bg200
import io.parity.signer.ui.theme.CryptoTypography
import io.parity.signer.ui.theme.Text300
import io.parity.signer.ui.theme.Typography
import org.json.JSONObject

/**
 * A card to show key info; only visual things.
 * TODO: paint root keys in scary colors
 */
@Composable
fun KeyCard(identity: JSONObject, signerDataModel: SignerDataModel) {
	Row(
		modifier = Modifier
			.padding(8.dp)
	) {
		Identicon(identity.getString("identicon"))
		Spacer(modifier = Modifier.width(10.dp))
		Column {
			Row {
				Text(
					identity.optString("path", identity.optString("derivation_path")),
					color = MaterialTheme.colors.onBackground,
					style = CryptoTypography.body2
				)
				if (identity.optBoolean("has_pwd", false)) {
					Text("///")
					Image(Icons.Default.Lock, contentDescription = "Locked account")
				}
			}
			Text(
				identity.optString("base58").abbreviateString(8),
				color = Text300,
				style = CryptoTypography.body2
			)
		}
	}
}

