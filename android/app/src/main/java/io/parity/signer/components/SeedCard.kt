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
	seedSelector: Boolean = true,
	signerDataModel: SignerDataModel
) {
	Row(
		modifier = Modifier
			.padding(8.dp)
	) {
		Image(
			identicon.intoImageBitmap(), "identicon", modifier = Modifier.scale(0.75f)
		)
		Spacer(modifier = Modifier.width(10.dp))
		Column {
			Text(seedName, color = Text600, style = MaterialTheme.typography.subtitle1)
		}
	}
}
