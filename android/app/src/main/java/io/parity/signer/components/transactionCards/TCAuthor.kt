package io.parity.signer.components.transactionCards

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.material.Icon
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Lock
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.scale
import io.parity.signer.models.SignerDataModel
import io.parity.signer.ui.theme.Typography
import org.json.JSONObject

@Composable
fun TCAuthor(payload: JSONObject, signerDataModel: SignerDataModel) {
	Row {
		//Image(signerDataModel.getIdenticon(payload.getString("base58"), 64), "identicon", modifier = Modifier.scale(0.75f))
		Column {

			Row {
				Text("From: ")
				Text(payload.getString("seed"), style = Typography.body1)
				Text(payload.getString("derivation_path").toString(), style = Typography.body1)
				if (payload.getBoolean("has_password")) Icon(Icons.Default.Lock, contentDescription = "Password protected account")
			}
			Text(payload.getString("base58"), style = Typography.body2)
			//Text(identity.get("public_key").toString(), style = Typography.body2)
		}
	}
}
