package io.parity.signer.screens.settings.networks.sufficientcrypto

import android.util.Log
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.parity.signer.bottomsheets.password.EnterPasswordModel
import io.parity.signer.bottomsheets.password.toEnterPasswordModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.backend.SignSufficientCryptoInteractor
import io.parity.signer.domain.storage.RepoResult
import io.parity.signer.domain.submitErrorState
import io.parity.signer.uniffi.MSignSufficientCrypto
import io.parity.signer.uniffi.MSufficientCryptoReady
import io.parity.signer.uniffi.ModalData
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch


class SignSufficientCryptoViewModel : ViewModel() {
	private val seedRepo = ServiceLocator.activityScope!!.seedRepository
	private val interactor = SignSufficientCryptoInteractor()

	private val _password: MutableStateFlow<EnterPasswordModel?> =
		MutableStateFlow(null)
	val password = _password.asStateFlow()
	private val _signature: MutableStateFlow<MSufficientCryptoReady?> = MutableStateFlow(null)
	val signature = _signature.asStateFlow()

	suspend fun getNetworkModel(networkKey: String): MSignSufficientCrypto? =
		interactor.signNetworkSpecs(networkKey)

	suspend fun getMetadataModel(
		networkKey: String,
		versionSpec: String
	): MSignSufficientCrypto? =
		interactor.signMetadataSpecInfo(networkKey, versionSpec)

	fun onSignSufficientCrypto(seedName: String, addressKey: String) {
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
					when (val modal = signResult?.modalData) {
						is ModalData.EnterPassword -> {
							_password.update { modal.f.toEnterPasswordModel() }
						}
						is ModalData.SufficientCryptoReady -> {
							_signature.update { modal.f }
						}
						else -> {
							submitErrorState("should be unreachable - sign succificnt crypto action")
						}
					}
				}
			}
		}
	}

	suspend fun passwordAttempt() {
		//todo dmitry
//		ios/PolkadotVault/Screens/Settings/Subviews/SignSpecs/SignSpecsListView.swift:148
	}

	fun isHasStateThenClear(): Boolean {
		return if (		password.value != null || signature.value != null) {
			clearState()
			true
		} else {
			false
		}
	}
	fun clearState() {
		_password.value = null
		_signature.value = null
	}
}
