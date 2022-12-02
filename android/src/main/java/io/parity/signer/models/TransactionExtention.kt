package io.parity.signer.models

import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.ActionResult
import io.parity.signer.uniffi.backendAction

suspend fun SignerDataModel.signTransaction(
	comment: String,
	seedNames: List<String>
): SignResult {
	return when (val authResult = authentication.authenticate(activity)) {
		AuthResult.AuthSuccess -> {
			val seedPhrases = seedNames
				.map { getSeed(it) }
				.filter { it.isNotEmpty() }
				.joinToString(separator = "/n")

			if (seedPhrases.isNotBlank()) {
				SignResult.Success(
					backendAction(Action.GO_FORWARD, comment, seedPhrases)
				)
			} else {
				SignResult.Failure(null)
			}
		}
		AuthResult.AuthError,
		AuthResult.AuthFailed,
		AuthResult.AuthUnavailable -> {
			SignResult.Failure(authResult)
		}
	}
}

sealed class SignResult {
	data class Success(val navResult: ActionResult) : SignResult()
	data class Failure(val auth: AuthResult?) : SignResult()
}
