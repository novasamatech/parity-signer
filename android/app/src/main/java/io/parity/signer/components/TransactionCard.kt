package io.parity.signer.components

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.components.transactionCards.*
import io.parity.signer.models.decodeHex
import io.parity.signer.models.toListOfStrings
import io.parity.signer.ui.theme.Text600
import io.parity.signer.uniffi.MTransaction
import org.json.JSONObject

/**
 * Selector for transaction card appearance
 */
@Composable
fun TransactionCard(card: MTransaction) {
	/*
	Box(
		modifier = Modifier
			.padding(start = (card.getInt("indent") * 10).dp)
			.fillMaxWidth()
	) {
		card.authorInfo?.let {
			TCAuthor(it)
		}
*/
		/*
		when (card.getString("type")) {
			"author" -> {
				TCAuthor(payload = payload)}
			"author_plain" -> {
				TCAuthorPlain(payload = payload)
			}
			"author_public_key" -> {
				TCAuthorPublicKey(payload = payload)
			}
			"balance" -> {
				TCBalance(currency = payload)
			}
			"bitvec" -> {
				TCBitVec(payload = payload)
			}
			"blockhash" -> {
				TCBlockHash(text = card.optString("payload"))
			}
			"default" -> {
				Text(card.getString("payload"), style = MaterialTheme.typography.body2, color = MaterialTheme.colors.Text600)
			}
			"derivations" -> {
				TCDerivations(payload = card.optJSONArray("payload")?.toListOfStrings() ?: listOf())
			}
			"enum_variant_name" -> {
				TCEnumVariantName(payload = payload)
			}
			"era" -> {
				if (payload.optString("era") == "Mortal") {
					TCEra(payload = payload)
				} else {
					TCEraImmortal()
				}
			}
			"error" -> {
				TCError(card.getString("payload"))
			}
			"field_name" -> {
				TCFieldName(payload = payload)
			}
			"field_number" -> {
				TCFieldNumber(payload = payload)
			}
			"Id" -> {
				TCID(payload = payload)
			}
			"identity_field" -> {
				TCIdentityField(text = card.optString("payload"))
			}
			"meta" -> {
				TCMeta(payload = payload)
			}
			"method" -> {
				TCMethod(card.getJSONObject("payload"))
			}
			"name_version" -> {
				TCNameVersion(payload = payload)
			}
			"network_genesis_hash" -> {
				TCGenesisHash(payload = card.optString("payload"))
			}
			"network_info" -> {
				// TODO: NetworkCard(deriveKey = payload)
			}
			"network_name" -> {
				TCNetworkName(text = card.optString("payload"))
			}
			"new_specs" -> {
				TCNewSpecs(payload = payload)
			}
			"nonce" -> {
				TCNonce(text = card.optString("payload"))
			}
			"none" -> {}
			"pallet" -> {
				TCPallet(card.getString("payload"))
			}
			"text" -> {
				Text(String(card.getString("payload").decodeHex()))
			}
			"tip" -> {
				TCTip(card.getJSONObject("payload"))
			}
			"tip_plain" -> {
				TCTipPlain(text = card.optString("payload"))
			}
			"tx_spec_plain" -> {
				TCTXSpecPlain(payload = payload)
			}
			"tx_version" -> {
				TCTXSpec(text = card.optString("payload"))
			}
			"types" -> {
				TCTypesInfo(payload = payload)
			}
			"varname" -> {
				TCVarName(card.optString("payload"))
			}
			"verifier" -> {
				TCVerifier(payload = payload)
			}
			"warning" -> {
				TCWarning(card.getString("payload"))
			}
			else -> {
				Row {
					Text(card.getString("type") + ": ")
					Text(card.optString("payload"))
				}
			}
		}
		*/
	//}
}
