package io.parity.signer.screens.scan.transaction

import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.models.AuthResult
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.getSeed
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.ActionResult
import io.parity.signer.uniffi.backendAction


class TransactionViewModel : ViewModel() {
	private val uniffiInteractor = ServiceLocator.backendLocator.uniffiInteractor
	private val authentication = ServiceLocator.authentication

	suspend fun signTransaction(
		comment: String,
		seedNames: List<String>,
		signerVM: SignerDataModel, //todo dmitry inbound get seed from it!
	): SignResult {
		return when (val authResult =
			ServiceLocator.authentication.authenticate(signerVM.activity)) {
			AuthResult.AuthSuccess -> {
				val seedPhrases = seedNames
					.map { signerVM.getSeed(it) }
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
}

sealed class SignResult {
	data class Success(val navResult: ActionResult) : SignResult()
	data class Failure(val auth: AuthResult?) : SignResult()
}
