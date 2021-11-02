package io.parity.signer.components

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.material.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.scale
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.abbreviateString
import io.parity.signer.ui.theme.Typography
import org.json.JSONObject

/**
 * A card to show key info; only visual things.
 * TODO: paint root keys in scary colors
 */
@Composable
fun KeyCard(identity: JSONObject, signerDataModel: SignerDataModel) {
	Row {
		Image(signerDataModel.getIdenticon(identity.get("ss58").toString(), 64), "identicon", modifier = Modifier.scale(0.75f))
		Column {
			Text(identity.get("path").toString(), style = Typography.body1)
			Text(identity.get("ss58").toString().abbreviateString(8), style = Typography.body2)
		}
	}
}
