package io.parity.signer.components

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.material.*
import androidx.compose.runtime.Composable
import io.parity.signer.models.SignerDataModel
import io.parity.signer.ui.theme.Typography
import org.json.JSONObject

/**
 * A card to show key info; only visual things.
 * TODO: paint root keys in scary colors
 */
@Composable
fun KeyCard(identity: JSONObject, signerDataModel: SignerDataModel) {
	Row {
		Image(signerDataModel.getIdenticon(identity.get("ss58").toString(), 80), "identicon")
		Column {
			Text(identity.get("name").toString())
			Row {
				Text(identity.get("seed_name").toString(), style = Typography.body1)
				Text(identity.get("path").toString(), style = Typography.body1)
			}
			Text(identity.get("ss58").toString(), style = Typography.body2)
			//Text(identity.get("public_key").toString(), style = Typography.body2)
		}
	}
}
