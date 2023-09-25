package io.parity.signer.screens.settings.networks.signspecs

import android.util.Log
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.parity.signer.bottomsheets.password.EnterPasswordModel
import io.parity.signer.bottomsheets.password.toEnterPasswordModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.NavigationError
import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.domain.backend.SignSufficientCryptoInteractor
import io.parity.signer.domain.storage.RepoResult
import io.parity.signer.domain.submitErrorState
import io.parity.signer.uniffi.ActionResult
import io.parity.signer.uniffi.ErrorDisplayed
import io.parity.signer.uniffi.ModalData
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch


class SignSpecsViewModel : ViewModel() {
	private val seedRepo = ServiceLocator.activityScope!!.seedRepository
	private val interactor = SignSufficientCryptoInteractor()

	private val _password: MutableStateFlow<EnterPasswordModel?> =
		MutableStateFlow(null)
	val password = _password.asStateFlow()
	private val _signature: MutableStateFlow<SignSpecsResultModel?> =
		MutableStateFlow(null)
	val signature = _signature.asStateFlow()

	suspend fun getKeysListModel(): OperationResult<SignSpecsListModel, ErrorDisplayed> =
		interactor.getSignCryptoKeys()

	fun onSignSpecs(
		input: SignSpecsInput,
		seedName: String,
		addressKey: String,
		password: String?,
	) {
		viewModelScope.launch {
			when (val seedResult = seedRepo.getSeedPhraseForceAuth(seedName)) {
				is RepoResult.Failure -> {
					Log.d(
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
							seedPhrase = seedName,
							password = password,
						)
						is SignSpecsInput.NetworkSpecs -> interactor.signNetworkWithKey(
							networkKey = input.networkKey,
							signingAddressKey = addressKey,
							seedPhrase = seedName,
							password = password,
						)
					}
					when (signResult) {
						is OperationResult.Err -> TODO() //todo dmitry
						is OperationResult.Ok -> when (val result = signResult.result) {
							SignSufficientCryptoInteractor.SignSpecsResult.PasswordWrong -> TODO()
							is SignSufficientCryptoInteractor.SignSpecsResult.Signature -> TODO()
						}
					}

					handleSignAttempt(signResult)
				}
			}
		}
	}

	fun requestPassword(		seedName: String,
													addressKey: String,) {

	}

	//todo dmitry remove below


	private fun onSignSufficientCrypto(seedName: String, addressKey: String) {
		viewModelScope.launch {
			when (val seedResult = seedRepo.getSeedPhraseForceAuth(seedName)) {
				is RepoResult.Failure -> {
					Log.d(
						"sufficient crypto",
						"failed to get seed to sign sufficient crypto"
					)
				}

				is RepoResult.Success -> {
					val signResult = interactor.attemptSigning(
						addressKey = addressKey,
						seedPhrase = seedResult.result
					)
					handleSignAttempt(signResult)
				}
			}
		}
	}

	private fun handleSignAttempt(signResult: OperationResult<ActionResult, NavigationError>) {
		when (signResult) {
			is OperationResult.Err -> {
				isHasStateThenClear()
				submitErrorState("should be unreachable - sign attepmt failed with error ${signResult.error}")
			}

			is OperationResult.Ok -> {
				when (val modal = signResult.result.modalData) {
					is ModalData.EnterPassword -> {
						_password.update { modal.f.toEnterPasswordModel() }
					}

					is ModalData.SufficientCryptoReady -> {
						_password.update { null }
						_signature.update { modal.f.toSignSpecsResultModel() }
					}

					else -> {
						isHasStateThenClear()
						//todo  show error for exceeded amout of attempts as in scan flow
						// use special api for this call
						submitErrorState("should be unreachable - sign succificnt crypto different result $signResult")
					}
				}
			}
		}
	}

	fun passwordAttempt(password: String) {
		viewModelScope.launch {
			val result = interactor.attemptPasswordEntered(password)
			handleSignAttempt(result)
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

	private fun clearState() {
		_password.value = null
		_signature.value = null
	}
}

sealed class SignSpecsInput {
	data class NetworkSpecs(val networkKey: String) : SignSpecsInput()
	data class NetworkMetadataSpecs(
		val networkKey: String,
		val versionSpec: String,
	) : SignSpecsInput()
}
