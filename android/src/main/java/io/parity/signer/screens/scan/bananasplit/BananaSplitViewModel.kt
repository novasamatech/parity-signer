package io.parity.signer.screens.scan.bananasplit

import android.content.Context
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.parity.signer.R
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.backend.AuthOperationResult
import io.parity.signer.domain.mapState
import io.parity.signer.domain.storage.SeedRepository
import io.parity.signer.domain.submitErrorState
import io.parity.signer.domain.usecases.CreateKeySetUseCase
import io.parity.signer.uniffi.BananaSplitRecoveryResult
import io.parity.signer.uniffi.DecodeSequenceResult
import io.parity.signer.uniffi.QrSequenceDecodeException
import io.parity.signer.uniffi.qrparserTryDecodeQrSequence
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

	//storing seed phrase between screens while user selecting networks
	private val _seedPhrase = MutableStateFlow<String?>(null)
	val isBananaRestorable = _seedPhrase.mapState(viewModelScope) { it != null }

	//ongoing events
	private val _password = MutableStateFlow<String>("")
	val password = _password.asStateFlow()
	private val _seedName = MutableStateFlow<String>("")
	val seedName = _seedName.asStateFlow()
	private val _seedCollision = MutableStateFlow<Boolean>(false)
	val seedCollision = _seedCollision.asStateFlow()
	private val _wrongPasswordCurrent = MutableStateFlow<Boolean>(false)
	val wrongPasswordCurrent = _wrongPasswordCurrent.asStateFlow()

	private val seedRepository: SeedRepository by lazy { ServiceLocator.activityScope!!.seedRepository }
	private val createKeySetUseCase = CreateKeySetUseCase()
	private lateinit var qrCodeData: List<String>
	private var invalidPasswordAttempts = 0

	fun initState(qrCodeData: List<String>) {
		cleanState()
		this.qrCodeData = qrCodeData
		this.invalidPasswordAttempts = 0
		_isWrongPasswordTerminal.value = false
	}

	fun cleanState() {
		_seedPhrase.value = null
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

	fun backToBananaRestore() {
		_seedPhrase.value = null
	}

	suspend fun onFinishWithNetworks(
		context: Context,
		networksKeys: Set<String>
	) {
		val seedName = seedName.value
		val seedPhrase = _seedPhrase.value!!
		val success = createKeySetUseCase.createKeySetWithNetworks(
			seedName, seedPhrase,
			networksKeys.toList(),
		)
		if (success == AuthOperationResult.Success) {
			_isCustomErrorTerminal.value =
				context.getString(R.string.banana_split_password_error_cannot_save_seed)
			return
		} else {
			//todo dmitry show error
		}
		_isSuccessTerminal.value = seedName
	}

	suspend fun onBananaDoneTry(context: Context) {
		val password = password.value

		try {
			when (val qrResult =
				qrparserTryDecodeQrSequence(qrCodeData, password, true)) {
				is DecodeSequenceResult.BBananaSplitRecoveryResult -> {
					when (val seedPhraseResult = qrResult.b) {
						is BananaSplitRecoveryResult.RecoveredSeed -> {
							if (seedRepository.isSeedPhraseCollision(seedPhraseResult.s)) {
								_isCustomErrorTerminal.value =
									context.getString(R.string.banana_split_password_error_seed_phrase_exists)
								return
							}
							_seedPhrase.value = seedPhraseResult.s
						}

						BananaSplitRecoveryResult.RequestPassword -> {
							submitErrorState("We passed password but recieved password request again, should be unreacheble ")
						}
					}
				}

				is DecodeSequenceResult.Other -> {
					submitErrorState("already processing banana split, but other qr code data happened to be here, submit it!, $qrResult")
				}

				is DecodeSequenceResult.DynamicDerivations -> {
					submitErrorState("already processing banana split, but other qr code data happened to be here, submit it!, $qrResult")
				}

				is DecodeSequenceResult.DynamicDerivationTransaction -> {
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
