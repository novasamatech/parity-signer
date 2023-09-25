package io.parity.signer.domain.backend

import io.parity.signer.R
import io.parity.signer.domain.FakeNavigator
import io.parity.signer.domain.NavigationError
import io.parity.signer.domain.Navigator
import io.parity.signer.screens.scan.errors.TransactionError
import io.parity.signer.screens.scan.errors.findErrorDisplayed
import io.parity.signer.screens.scan.errors.toTransactionError
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.ActionResult
import io.parity.signer.uniffi.ErrorDisplayed
import io.parity.signer.uniffi.backendAction
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext


/**
 * Part of Uniffi logic used in scan flow because
 */
class ScanFlowInteractor {

	private val navigator: Navigator = FakeNavigator()

	private suspend fun navigate(
		action: Action,
		details: String = "",
		seedPhrase: String = "",
	): OperationResult<ActionResult, NavigationError> =
		withContext(Dispatchers.IO) {
			try {
				OperationResult.Ok(backendAction(action, details, seedPhrase))
			} catch (e: ErrorDisplayed) {
				OperationResult.Err(
					NavigationError(
						e.findErrorDisplayed()?.message ?: e.message
						?: "unknown navigation error"
					)
				)
			}
		}

	suspend fun resetMachineState() {
		navigator.navigate(Action.START)
		navigator.navigate(Action.NAVBAR_SCAN)
	}

	suspend fun continueSigningTransaction(
		comment: String,
		seedPhrases: String
	): ActionResult? {
		return navigate(
			Action.GO_FORWARD,
			comment,
			seedPhrases
		).mapError()
	}

	suspend fun handlePasswordEntered(password: String): OperationResult<ActionResult, NavigationError> {
		return navigate(
			Action.GO_FORWARD,
			password,
		)
	}

	suspend fun performTransaction(payload: String): OperationResult<ActionResult, TransactionError> {
		resetMachineState()
		return try {
			OperationResult.Ok(
				backendAction(
					Action.TRANSACTION_FETCHED,
					payload,
					""
				)
			)
		} catch (e: ErrorDisplayed) {
			OperationResult.Err(e.toTransactionError())
		} catch (e: Throwable) {
			OperationResult.Err(
				TransactionError.Generic(
					e.findErrorDisplayed()?.message ?: e.message ?: ""
				)
			)
		}
	}
}
