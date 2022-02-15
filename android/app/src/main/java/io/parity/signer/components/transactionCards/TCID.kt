package io.parity.signer.components.transactionCards

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.scale
import io.parity.signer.components.Identicon
import io.parity.signer.models.SignerDataModel
import org.json.JSONObject

@Composable
fun TCID(payload: JSONObject) {
	Row {
		Identicon(identicon = payload.optString("identicon"))
		Column {
			Text(payload.optString("base58"))
		}
	}
}
