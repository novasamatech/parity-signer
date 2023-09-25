package io.parity.signer.domain.backend

import io.parity.signer.domain.FakeNavigator
import io.parity.signer.domain.NavigationError
import io.parity.signer.domain.Navigator
import io.parity.signer.screens.scan.errors.findErrorDisplayed
import io.parity.signer.screens.settings.networks.signspecs.SignSpecsListModel
import io.parity.signer.screens.settings.networks.signspecs.toSignSpecsListModel
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.ActionResult
import io.parity.signer.uniffi.AlertData
import io.parity.signer.uniffi.ErrorDisplayed
import io.parity.signer.uniffi.MRawKey
import io.parity.signer.uniffi.MSignSufficientCrypto
import io.parity.signer.uniffi.ScreenData
import io.parity.signer.uniffi.backendAction
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext


/**
 * Part of Uniffi logic used in scan flow because
 */
class SignSufficientCryptoInteractor {

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

	fun resetMachineState(networkKey: String) {
		navigator.navigate(Action.START)
		navigator.navigate(Action.NAVBAR_SETTINGS)
		navigator.navigate(Action.MANAGE_NETWORKS)
		navigator.navigate(Action.GO_FORWARD, networkKey)
	}

	fun closedBottomSheet() {
		navigator.navigate(Action.GO_BACK)
	}

	suspend fun signNetworkSpecs(
		networkKey: String,
	): OperationResult<SignSpecsListModel, Any> {
		resetMachineState(networkKey)
		navigator.navigate(Action.RIGHT_BUTTON_ACTION)
		val result = navigate(
			Action.SIGN_NETWORK_SPECS,
		).map {
			val successful =
				(it.screenData as? ScreenData.SignSufficientCrypto)?.f?.toSignSpecsListModel()
			return@map if (successful != null) {
				OperationResult.Ok(successful)
			} else {
				if (it.alertData is AlertData.ErrorData) {
					OperationResult.Err(NavigationError("Rust alert error is ${(it.alertData as AlertData.ErrorData).f}"))
				}else {
					OperationResult.Err("Unknown navigation, full object is $it")
				}
			}
		}
		return result
	}

	suspend fun signMetadataSpecInfo(
		networkKey: String,
		specsVersion: String,
	): OperationResult<SignSpecsListModel, Any> {
		resetMachineState(networkKey)
		navigator.navigate(Action.MANAGE_METADATA, specsVersion)
		val result = navigate(
			Action.SIGN_METADATA,
		).map {
			val successful =
				(it.screenData as? ScreenData.SignSufficientCrypto)?.f?.toSignSpecsListModel()
			return@map if (successful != null) {
				OperationResult.Ok(successful)
			} else {
				if (it.alertData is AlertData.ErrorData) {
					OperationResult.Err(NavigationError("Rust alert error is ${(it.alertData as AlertData.ErrorData).f}"))
				}else {
					OperationResult.Err("Unknown navigation, full object is $it")
				}
			}
		}
		return result
	}

	suspend fun attemptSigning(
		addressKey: String,
		seedPhrase: String
	): OperationResult<ActionResult, NavigationError> {
		return navigate(
			Action.GO_FORWARD,
			addressKey,
			seedPhrase,
		)
	}

	suspend fun attemptPasswordEntered(password: String): OperationResult<ActionResult, NavigationError> {
		return navigate(
			Action.GO_FORWARD,
			password,
		)
	}

}
