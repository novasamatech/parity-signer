package io.parity.signer.domain.backend

import io.parity.signer.domain.FakeNavigator
import io.parity.signer.domain.NavigationError
import io.parity.signer.domain.Navigator
import io.parity.signer.screens.scan.errors.findErrorDisplayed
import io.parity.signer.screens.settings.networks.signspecs.SignSpecsListModel
import io.parity.signer.screens.settings.networks.signspecs.SignSpecsResultModel
import io.parity.signer.screens.settings.networks.signspecs.toSignSpecsListModel
import io.parity.signer.screens.settings.networks.signspecs.toSignSpecsResultModel
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.ActionResult
import io.parity.signer.uniffi.AlertData
import io.parity.signer.uniffi.ErrorDisplayed
import io.parity.signer.uniffi.ScreenData
import io.parity.signer.uniffi.backendAction
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext


/**
 * Part of Uniffi logic used in scan flow because
 */
class SignSufficientCryptoInteractor {

	suspend fun getSignCryptoKeys(
	): OperationResult<SignSpecsListModel, ErrorDisplayed> =
		withContext(Dispatchers.IO) {
			try {
				val keysList =
					io.parity.signer.uniffi.getKeysForSigning().toSignSpecsListModel()
				OperationResult.Ok(keysList)
			} catch (e: ErrorDisplayed) {
				OperationResult.Err(e)
			}
		}

	suspend fun signNetworkMetadataWithKey(
		networkKey: String,
		metadataSpecsVersion: String,
		signingAddressKey: String,
		seedPhrase: String,
		password: String?
	): OperationResult<SignSpecsResult, ErrorDisplayed> =
		withContext(Dispatchers.IO) {
			try {
				val signature = io.parity.signer.uniffi.signMetadataWithKey(
					networkKey,
					metadataSpecsVersion,
					signingAddressKey,
					seedPhrase,
					password,
				).toSignSpecsResultModel()
				OperationResult.Ok(SignSpecsResult.Signature(signature))
			} catch (e: ErrorDisplayed) {
				when (e) {
					is ErrorDisplayed.WrongPassword -> OperationResult.Ok(SignSpecsResult.PasswordWrong)
					else -> OperationResult.Err(e)
				}
			}
		}

	suspend fun signNetworkWithKey(
		networkKey: String,
		signingAddressKey: String,
		seedPhrase: String,
		password: String?,
	): OperationResult<SignSpecsResult, ErrorDisplayed> =
		withContext(Dispatchers.IO) {
			try {
				val signature = io.parity.signer.uniffi.signNetworkSpecWithKey(
					networkKey,
					signingAddressKey,
					seedPhrase,
					password,
				).toSignSpecsResultModel()
				OperationResult.Ok(SignSpecsResult.Signature(signature))
			} catch (e: ErrorDisplayed) {
				when (e) {
					is ErrorDisplayed.WrongPassword -> OperationResult.Ok(SignSpecsResult.PasswordWrong)
					else -> OperationResult.Err(e)
				}
			}
		}

	sealed class SignSpecsResult {
		class Signature(val result: SignSpecsResultModel) : SignSpecsResult()
		object PasswordWrong : SignSpecsResult()
	}
}
