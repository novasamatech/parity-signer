package io.parity.signer.screens.scan.bananasplit

import android.content.Context
import androidx.lifecycle.ViewModel
import io.parity.signer.R
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.FakeNavigator
import io.parity.signer.domain.storage.SeedRepository
import io.parity.signer.domain.submitErrorState
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


	private val uniffiInteractor = ServiceLocator.uniffiInteractor
	private val seedRepository: SeedRepository by lazy { ServiceLocator.activityScope!!.seedRepository }
	private lateinit var qrCodeData: List<String>
	private var invalidPasswordAttempts = 0

	fun initState(qrCodeData: List<String>) {
		cleanState()
		this.qrCodeData = qrCodeData
		this.invalidPasswordAttempts = 0
		_isWrongPasswordTerminal.value = false
	}

	fun cleanState() {
		_password.value = ""
		_seedName.value = ""
		_seedCollision.value = false
		_wrongPasswordCurrent.value = false

		_isWrongPasswordTerminal.value = false
		_isSuccessTerminal.value = null
		_isCustomErrorTerminal.value = null
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

	suspend fun onDoneTap(context: Context) {
		val password = password.value
		val seedName = seedName.value
		try {
			when (val qrResult =
				qrparserTryDecodeQrSequence(qrCodeData, password, true)) {
				is DecodeSequenceResult.BBananaSplitRecoveryResult -> {
					when (val seed = qrResult.b) {
						is BananaSplitRecoveryResult.RecoveredSeed -> {
							if (seedRepository.isSeedPhraseCollision(seed.s)) {
								_isCustomErrorTerminal.value =
									context.getString(R.string.banana_split_password_error_seed_phrase_exists)
								return
							}
							val fakeNavigator = FakeNavigator()
							//fake navigations
							fakeNavigator.navigate(Action.NAVBAR_KEYS)
							// Key Set List state has different "modalData" state depending on whether user has at least one key or not
							// So we need to check whether we should actually "pretend" to open "more" navigation bar menu by
							if (seedRepository.getLastKnownSeedNames().isNotEmpty()) {
								fakeNavigator.navigate(Action.RIGHT_BUTTON_ACTION)
							}
							fakeNavigator.navigate(Action.RECOVER_SEED)
							fakeNavigator.navigate(Action.GO_FORWARD, seedName)
							// We should do additional check on whether seed can be successfully saved and not call navigation
							// further if there are any issues (i.e. somehow seedname is still empty, etc)
							val isSaved = seedRepository.addSeed(
								seedName = seedName,
								seedPhrase = seed.s,
								navigator = FakeNavigator(),
								isOptionalAuth = true,
							)
							if (!isSaved) {
								_isCustomErrorTerminal.value =
									context.getString(R.string.banana_split_password_error_cannot_save_seed)
								return
							}
							fakeNavigator.navigate(Action.GO_BACK)
							_isSuccessTerminal.value = seedName
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
					_wrongPasswordCurrent.value = true
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
