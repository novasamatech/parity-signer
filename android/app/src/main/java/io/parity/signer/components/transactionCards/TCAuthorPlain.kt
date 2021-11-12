package io.parity.signer.components.transactionCards

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import io.parity.signer.models.SignerDataModel
import io.parity.signer.ui.theme.Typography
import org.json.JSONObject

@Composable
fun TCAuthorPlain(payload: JSONObject, signerDataModel: SignerDataModel) {
	Row {
		Image(signerDataModel.getIdenticon(payload.getString("base58"), 80), "identicon")
		Column {
			Text("From: ")
			Text(payload.getString("base58"), style = Typography.body2)
			//Text(identity.get("public_key").toString(), style = Typography.body2)
		}
	}
}
