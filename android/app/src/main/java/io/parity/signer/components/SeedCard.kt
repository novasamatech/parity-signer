package io.parity.signer.components

import android.util.Log
import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.*
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.scale
import androidx.compose.ui.unit.dp
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.abbreviateString
import io.parity.signer.models.intoImageBitmap
import io.parity.signer.ui.theme.CryptoTypography
import io.parity.signer.ui.theme.Text300
import io.parity.signer.ui.theme.Text600
import io.parity.signer.ui.theme.Typography
import org.json.JSONObject

@Composable
fun SeedCard(
	seedName: String,
	identicon: String,
	base58: String = "",
	showAddress: Boolean = false
) {
	Row(
		modifier = Modifier
			.padding(8.dp)
	) {
		Identicon(identicon)
		Spacer(modifier = Modifier.width(10.dp))
		Column {
			Text(
				seedName,
				color = MaterialTheme.colors.Text600,
				style = MaterialTheme.typography.subtitle1
			)
			if (showAddress) {
				Text(base58)
			}
		}
	}
}
