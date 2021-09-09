package io.parity.signer.components

import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.RectangleShape
import androidx.compose.ui.unit.dp
import io.parity.signer.components.TransactionCards.*
import io.parity.signer.models.SignerDataModel
import org.json.JSONObject

@Composable
fun TransactionCard(card: JSONObject, signerDataModel: SignerDataModel) {
	Box (modifier = Modifier
		.padding(start = (card.getInt("indent") * 10).dp)
		.fillMaxWidth()
		.border(1.dp, Color.Red)) {
		when (card.getString("type")) {
			"author" -> {
				TCAuthor(card.getJSONObject("payload"), signerDataModel)
			}
			"author_plain" -> {
				TCAuthorPlain(card.getJSONObject("payload"), signerDataModel)
			}
			"balance" -> {
				TCBalance(card.getJSONObject("payload"))
			}
			"call" -> {
				TCCall(card.getJSONObject("payload"))
			}
			"enum_variant_name" -> {Text(card.getString("payload"))}
			"era_immortal_nonce" -> {
				TCEraImmortalNonce(card.getJSONObject("payload"))
			}
			"era_mortal_nonce" -> {
				TCEraMortalNonce(card.getJSONObject("payload"))
			}
			"error" -> {
				TCError(card.getString("payload"))
			}
			"Id" -> {
				TCID(card.getString("payload"), signerDataModel)
			}
			"tip" -> {
				TCTip(card.getJSONObject("payload"))
			}
			"tx_spec" -> {
				TCETXSpec(card.getJSONObject("payload"))
			}
			"varname" -> {Text(card.getString("payload"))}
			"warning" -> {
				TCWarning(card.getString("payload"))
			}
 			"default" -> {
				Text(card.getString("payload"))
			}
			else -> {
				Row {
					Text(card.getString("type") + ": ")
					Text(card.optString("payload"))
				}
			}
		}
	}
}
