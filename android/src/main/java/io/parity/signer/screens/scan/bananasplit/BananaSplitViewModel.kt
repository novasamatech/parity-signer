package io.parity.signer.screens.scan.bananasplit

import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.models.FakeNavigator
import io.parity.signer.models.storage.SeedRepository
import io.parity.signer.models.submitErrorState
import io.parity.signer.uniffi.*
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow


class BananaSplitViewModel() : ViewModel() {

	private val _isWrongPassword = MutableStateFlow(false)
	val isWrongPassword = _isWrongPassword.asStateFlow()

	private val _isError = MutableStateFlow<String?>(null)
	val isError = _isError.asStateFlow()
	private val _isSuccess = MutableStateFlow<String?>(null)
	val isSuccess = _isSuccess.asStateFlow()

	private val uniffiInteractor = ServiceLocator.backendScope.uniffiInteractor
	private val seedRepository: SeedRepository by lazy { ServiceLocator.activityScope!!.seedRepository }
	private lateinit var qrCodeData: List<String>
	private var invalidPasswordAttempts = 0

	fun initState(qrCodeData: List<String>) {
		this.qrCodeData = qrCodeData
		this.invalidPasswordAttempts = 0
		_isWrongPassword.value = false
	}

	fun isPathCollision(): Boolean {
		return false
	}

	suspend fun onDoneTap(seedName: String, password: String) {
		try {
			when (val qrResult =
				qrparserTryDecodeQrSequence(qrCodeData, password, false)) {
				is DecodeSequenceResult.BBananaSplitRecoveryResult -> {
					when (val seed = qrResult.b) {
						is BananaSplitRecoveryResult.RecoveredSeed -> {
							if (seedRepository.isSeedPhraseCollision(seed.s)) {
//								dismissWithError(.seedPhraseAlreadyExists())
//						return
							}
							//fake navigations
							uniffiInteractor.navigate(Action.NAVBAR_KEYS)
							uniffiInteractor.navigate(Action.RIGHT_BUTTON_ACTION)
							uniffiInteractor.navigate(Action.RECOVER_SEED)
							uniffiInteractor.navigate(Action.GO_FORWARD, seedName)
							seedRepository.addSeed(
								seedName = seedName,
								seedPhrase = seed.s,
								navigator = FakeNavigator(),
								isOptionalAuth = true
							)
							uniffiInteractor.navigate(Action.GO_BACK)
							_isSuccess.value = seedName
							//todo banana
//					navigation.overrideQRScannerDismissalNavigation = .init(action: .selectSeed, details: seedName)
						}
						BananaSplitRecoveryResult.RequestPassword -> {
							submitErrorState("We passed password but recieved password request again, should be unreacheble ")
						}
					}
				}
				is DecodeSequenceResult.Other -> {
					submitErrorState("already processing banana split, but other qr code data happened to be here, submit it!, $qrResult")
				}
			}
		} catch (e: QrSequenceDecodeException) {
			when (e) {
				is QrSequenceDecodeException.BananaSplitWrongPassword -> {
					invalidPasswordAttempts += 1
					if (invalidPasswordAttempts > 3) {
//						todo banana
//						dismissWithError(.signingForgotPassword())
//					return
					}
					_isWrongPassword.value = true
				}
				is QrSequenceDecodeException.BananaSplit -> {
					val error = e.s
//					dismissWithError(.alertError(message: errorDetail))
				}
				is QrSequenceDecodeException.GenericException -> {
					val error = e.s
//					dismissWithError(.alertError(message: errorDetail))
				}
			}
		}
	}
}
