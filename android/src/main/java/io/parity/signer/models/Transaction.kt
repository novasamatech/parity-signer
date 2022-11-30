package io.parity.signer.models

import io.parity.signer.uniffi.Action

fun SignerDataModel.signTransaction(
	comment: String,
	seedNames: List<String>
) {
	authentication.authenticate(activity) {
		val seedPhrases = seedNames
			.map { getSeed(it) }
			.filter { it.isNotEmpty() }
			.joinToString(separator = "/n")

		if (seedPhrases.isNotBlank()) {
			//todo dmitry it will open new transaction so should open still in camera view
			navigate(Action.GO_FORWARD, comment, seedPhrases)
		}
	}
}

fun SignerDataModel.signSufficientCrypto(seedName: String, addressKey: String) {
	authentication.authenticate(activity) {
		val seedPhrase = getSeed(
			seedName
		)
		if (seedPhrase.isNotBlank()) {
			navigate(
				Action.GO_FORWARD,
				addressKey,
				seedPhrase
			)
		}
	}
}
