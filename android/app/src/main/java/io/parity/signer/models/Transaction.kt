package io.parity.signer.models

import io.parity.signer.uniffi.Action
import org.json.JSONObject


fun SignerDataModel.signTransaction(
	comment: String,
	seedName: String
) {
	authentication.authenticate(activity) {
		val seedPhrase = getSeed(
			seedName
		)
		if (seedPhrase.isNotBlank()) {
			pushButton(Action.GO_FORWARD, comment.encode64(), seedPhrase)
		}
	}
}

fun SignerDataModel.signSufficientCrypto(identity: JSONObject) {
	authentication.authenticate(activity) {
		val seedPhrase = getSeed(
			identity.optString("seed_name")
		)
		if (seedPhrase.isNotBlank()) {
			pushButton(
				Action.GO_FORWARD,
				identity.optString("address_key"),
				seedPhrase
			)
		}
	}
}
