package io.parity.signer.backend

import io.parity.signer.uniffi.*

/**
 * Wrapper for uniffi calls into rust. Made for centralized handling errors
 * and to have those functions scoped in specific namespace
 */
object UniffiInteractor {

	fun navigate(
		action: Action,
		details: String,
		seedPhrase: String
	): Result<ActionResult> {
		return try {
			Result.success(backendAction(action, details, seedPhrase))
		} catch (e: ErrorDisplayed) {
			Result.failure(e)
		}
	}

	//todo dmitry get qr codes for export
}
