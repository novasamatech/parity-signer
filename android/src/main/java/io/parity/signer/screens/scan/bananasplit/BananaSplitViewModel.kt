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

	//terminal events
	private val _isWrongPasswordTerminal = MutableStateFlow(false)
	val isWrongPasswordTerminal = _isWrongPasswordTerminal.asStateFlow()
	private val _isCustomErrorTerminal = MutableStateFlow<String?>(null)
	val isCustomErrorTerminal = _isCustomErrorTerminal.asStateFlow()
	private val _isSuccessTerminal = MutableStateFlow<String?>(null)
	//String is seed name
	val isSuccessTerminal = _isSuccessTerminal.asStateFlow()

	//ongoing events
	private val _password = MutableStateFlow<String>("")
	val password = _password.asStateFlow()
	private val _seedName = MutableStateFlow<String>("")
	val seedName = _seedName.asStateFlow()
	private val _seedCollision = MutableStateFlow<Boolean>(false)
	val seedCollision = _seedCollision.asStateFlow()
	private val _wrongPasswordCurrent = MutableStateFlow<Boolean>(false)
	val wrongPasswordCurrent = _wrongPasswordCurrent.asStateFlow()


	private val uniffiInteractor = ServiceLocator.backendScope.uniffiInteractor
	private val seedRepository: SeedRepository by lazy { ServiceLocator.activityScope!!.seedRepository }
	private lateinit var qrCodeData: List<String>
	private var invalidPasswordAttempts = 0

	fun initState(qrCodeData: List<String>) {
		this.qrCodeData = qrCodeData
		this.invalidPasswordAttempts = 0
		_isWrongPasswordTerminal.value = false
	}

	fun updatePassword(newPassword: String) {
		if (wrongPasswordCurrent.value && newPassword != password.value) {
			_wrongPasswordCurrent.value = false
		}
		_password.value = newPassword
	}

	fun updateSeedName(newSeedName: String) {
		_seedCollision.value = seedRepository.containSeedName(newSeedName)
		_seedName.value = newSeedName
	}

	suspend fun onDoneTap() {
		val password = password.value
		val seedName = seedName.value
		try {
			when (val qrResult =
				qrparserTryDecodeQrSequence(qrCodeData, password, false)) {
				is DecodeSequenceResult.BBananaSplitRecoveryResult -> {
					when (val seed = qrResult.b) {
						is BananaSplitRecoveryResult.RecoveredSeed -> {
							if (seedRepository.isSeedPhraseCollision(seed.s)) {
								_isCustomErrorTerminal.value = "This seed phrase already exists"
								return
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
							_isSuccessTerminal.value = seedName
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
						_isWrongPasswordTerminal.value = true
					return
					}
					_isWrongPasswordTerminal.value = true
				}
				is QrSequenceDecodeException.BananaSplit -> {
					val error = e.s
					_isCustomErrorTerminal.value = error
				}
				is QrSequenceDecodeException.GenericException -> {
					val error = e.s
					_isCustomErrorTerminal.value = error
				}
			}
		}
	}
}
