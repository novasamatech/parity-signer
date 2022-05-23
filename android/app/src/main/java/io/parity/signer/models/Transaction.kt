package io.parity.signer.models

import io.parity.signer.uniffi.Action


fun SignerDataModel.signTransaction(
	comment: String,
	seedName: String
) {
	authentication.authenticate(activity) {
		val seedPhrase = getSeed(
			seedName
		)
		if (seedPhrase.isNotBlank()) {
			pushButton(Action.GO_FORWARD, comment, seedPhrase)
		}
	}
}

fun SignerDataModel.signSufficientCrypto(seedName: String, addressKey: String) {
	authentication.authenticate(activity) {
		val seedPhrase = getSeed(
			seedName
		)
		if (seedPhrase.isNotBlank()) {
			pushButton(
				Action.GO_FORWARD,
				addressKey,
				seedPhrase
			)
		}
	}
}
