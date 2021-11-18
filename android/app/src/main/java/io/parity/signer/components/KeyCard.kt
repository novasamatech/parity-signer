package io.parity.signer.components

import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.scale
import androidx.compose.ui.unit.dp
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.abbreviateString
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
	Row (modifier = Modifier
		.padding(8.dp)) {
		Image(signerDataModel.getIdenticon(identity.get("ss58").toString(), 64), "identicon", modifier = Modifier.scale(0.75f))
		Spacer(modifier = Modifier.width(10.dp))
		Column {
			Text(identity.get("path").toString(), color = MaterialTheme.colors.onBackground, style = CryptoTypography.body2)
			Text(identity.get("ss58").toString().abbreviateString(8), color = Text300, style = CryptoTypography.body2)
		}
	}
}
