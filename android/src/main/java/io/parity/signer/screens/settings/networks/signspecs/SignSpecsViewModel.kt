package io.parity.signer.screens.settings.networks.signspecs

import timber.log.Timber
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.parity.signer.R
import io.parity.signer.bottomsheets.password.EnterPasswordModel
import io.parity.signer.components.sharedcomponents.KeyCardModelBase
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.domain.backend.SignSufficientCryptoInteractor
import io.parity.signer.domain.getDebugDetailedDescriptionString
import io.parity.signer.domain.storage.RepoResult
import io.parity.signer.screens.scan.errors.LocalErrorSheetModel
import io.parity.signer.uniffi.ErrorDisplayed
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch


class SignSpecsViewModel : ViewModel() {
	private val appContext
		get() = ServiceLocator.appContext
	private val seedRepo
		get() = ServiceLocator.activityScope!!.seedRepository
	private val interactor = SignSufficientCryptoInteractor()

	private val _password: MutableStateFlow<PasswordState?> =
		MutableStateFlow(null)
	internal val password = _password.asStateFlow()
	private val _signature: MutableStateFlow<SignSpecsResultModel?> =
		MutableStateFlow(null)
	val signature = _signature.asStateFlow()
	private val _localError: MutableStateFlow<LocalErrorSheetModel?> =
		MutableStateFlow(null)
	val localError = _localError.asStateFlow()


	suspend fun getKeysListModel(): OperationResult<SignSpecsListModel, ErrorDisplayed> =
		interactor.getSignCryptoKeys()

	fun onSignSpecs(
		input: SignSpecsInput,
		keyModel: KeyCardModelBase,
		addressKey: String,
		password: String?,
	) {
		viewModelScope.launch {
			when (val seedResult =
				seedRepo.getSeedPhraseForceAuth(keyModel.seedName)) {
				is RepoResult.Failure -> {
					Timber.d(
						"sufficient crypto",
						"failed to get seed to sign sufficient crypto"
					)
				}

				is RepoResult.Success -> {
					val signResult = when (input) {
						is SignSpecsInput.NetworkMetadataSpecs -> interactor.signNetworkMetadataWithKey(
							networkKey = input.networkKey,
							metadataSpecsVersion = input.versionSpec,
							signingAddressKey = addressKey,
							seedPhrase = seedResult.result,
							password = password,
						)

						is SignSpecsInput.NetworkSpecs -> interactor.signNetworkWithKey(
							networkKey = input.networkKey,
							signingAddressKey = addressKey,
							seedPhrase = seedResult.result,
							password = password,
						)
					}
					when (signResult) {
						is OperationResult.Err -> {
							_localError.value = LocalErrorSheetModel(
								title = appContext.getString(R.string.sign_specs_error_signing_title),
								subtitle = signResult.error.getDebugDetailedDescriptionString(),
							)
						}

						is OperationResult.Ok -> when (val result = signResult.result) {
							SignSufficientCryptoInteractor.SignSpecsResult.PasswordWrong -> {
								requestPassword(
									keyModel = keyModel,
									addressKey = addressKey,
								)
							}

							is SignSufficientCryptoInteractor.SignSpecsResult.Signature -> {
								_signature.update { result.result }
								_password.update { null }
							}
						}
					}
					System.gc()
				}
			}
		}
	}

	fun requestPassword(
		keyModel: KeyCardModelBase,
		addressKey: String,
	) {
		_password.update {
			if (it == null) {
				PasswordState(
					model = EnterPasswordModel(keyModel, false, 0),
					addressKey64 = addressKey,
				)
			} else if (it.model.attempt > 3) {
				_localError.value = LocalErrorSheetModel(
					title = appContext.getString(R.string.attempts_exceeded_title),
					subtitle = appContext.getString(R.string.attempts_exceeded_message),
				)
				null
			} else {
				it.copy(
					model = it.model.copy(
						showError = true,
						attempt = it.model.attempt + 1,
					)
				)
			}
		}
	}

	fun isHasStateThenClear(): Boolean {
		return if (password.value != null || signature.value != null) {
			clearState()
			true
		} else {
			false
		}
	}

	fun clearError() {
		_localError.value = null
	}

	private fun clearState() {
		_password.value = null
		_signature.value = null
		_localError.value = null
	}
}

internal data class PasswordState(
	val model: EnterPasswordModel,
	val addressKey64: String
)

sealed class SignSpecsInput {
	data class NetworkSpecs(val networkKey: String) : SignSpecsInput()
	data class NetworkMetadataSpecs(
		val networkKey: String,
		val versionSpec: String,
	) : SignSpecsInput()
}
