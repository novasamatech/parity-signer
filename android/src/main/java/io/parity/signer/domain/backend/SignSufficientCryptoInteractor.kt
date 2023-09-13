package io.parity.signer.domain.backend

import io.parity.signer.domain.FakeNavigator
import io.parity.signer.domain.NavigationError
import io.parity.signer.domain.Navigator
import io.parity.signer.screens.scan.errors.findErrorDisplayed
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.ActionResult
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

	private suspend fun resetMachineState(networkKey: String) {
		navigator.navigate(Action.START)
		navigator.navigate(Action.NAVBAR_SETTINGS)
		navigator.navigate(Action.MANAGE_NETWORKS)
	}

	//todo dmitry as ios/PolkadotVault/Backend/NavigationServices/ManageNetworkDetailsService.swift:30
	suspend fun signNetworkSpecs(
		networkKey: String,
	): MSignSufficientCrypto? {
		resetMachineState(networkKey)
		navigator.navigate(Action.RIGHT_BUTTON_ACTION)
		val result = navigate(
			Action.SIGN_NETWORK_SPECS,
		).mapError()
		return (result?.screenData as? ScreenData.SignSufficientCrypto)?.f
	}

	suspend fun signMetadataSpecInfo(
		networkKey: String,
		specsVersion: String,
	): MSignSufficientCrypto? {
		resetMachineState(networkKey)
		navigator.navigate(Action.MANAGE_METADATA, specsVersion)
		val result = navigate(
			Action.SIGN_METADATA,
		).mapError()
		return (result?.screenData as? ScreenData.SignSufficientCrypto)?.f
	}

	suspend fun attemptSigning(
		keyRecord: MRawKey,
		seedPhrase: String
	): ActionResult? {
		return navigate(
			Action.GO_FORWARD,
			keyRecord.addressKey,
			seedPhrase,
		).mapError()
	}

	suspend fun attemptPasswordEntered(password: String): OperationResult<ActionResult, NavigationError> {
		return navigate(
			Action.GO_FORWARD,
			password,
		)
	}

}
