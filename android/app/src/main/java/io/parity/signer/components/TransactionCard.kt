package io.parity.signer.components

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.components.transactionCards.*
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.decodeHex
import org.json.JSONObject

@Composable
fun TransactionCard(card: JSONObject, signerDataModel: SignerDataModel) {
	Box(
		modifier = Modifier
			.padding(start = (card.getInt("indent") * 10).dp)
			.fillMaxWidth()
	) {
		when (card.getString("type")) {
			"balance" -> {
				TCBalance(card.getJSONObject("payload"))
			}
			"call" -> {
				TCCall(card.getJSONObject("payload"))
			}
			"enum_variant_name" -> {
				Text(card.getJSONObject("payload").getString("name"))
			}
			"era" -> {
				TCEra(card.getJSONObject("payload"))
			}
			"error" -> {
				TCError(card.getString("payload"))
			}
			"field_name" -> {
				Text(card.getJSONObject("payload").getString("name"))
			}
			"Id" -> {
				TCID(card.getString("payload"), signerDataModel)
			}
			"method" -> {
				TCMethod(card.getJSONObject("payload"))
			}
			"name_version" -> {
				Text(
					card.getJSONObject("payload")
						.getString("name") + " v " + card.getJSONObject("payload")
						.getString("version")
				)
			}
			"pallet" -> {
				Text("Pallet: " + card.getString("payload"))
			}
			"text" -> {
				Text(String(card.getString("payload").decodeHex()))
			}
			"tip" -> {
				TCTip(card.getJSONObject("payload"))
			}
			"tx_spec" -> {
				TCETXSpec(card.getJSONObject("payload"))
			}
			"varname" -> {
				Text(card.getString("payload"))
			}
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
