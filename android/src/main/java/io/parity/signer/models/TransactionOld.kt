package io.parity.signer.models

import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.uniffi.Action

fun SignerDataModel.signSufficientCrypto(seedName: String, addressKey: String) {
	ServiceLocator.authentication.authenticate(activity) {
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
