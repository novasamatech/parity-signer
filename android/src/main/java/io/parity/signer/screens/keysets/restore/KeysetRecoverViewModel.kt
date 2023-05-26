package io.parity.signer.screens.keysets.restore

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.parity.signer.backend.OperationResult
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.Navigator
import io.parity.signer.domain.submitErrorState
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.ScreenData
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking


class KeysetRecoverViewModel : ViewModel() {

	private val _recoverState = MutableStateFlow<KeysetRecoverModel?>(null)
	val recoverState = _recoverState.asStateFlow()

	fun initValue(model: KeysetRecoverModel) {
		if (_recoverState.value == null) {
			_recoverState.value = model
		}
	}

	/**
	 * Ideally navigation scope should reset this VM, but since this screen
	 * is handled in rust-navigation state machine, need to clean state manually for now
	 */
	fun resetState() {
		_recoverState.value = null
	}

	fun addSeed(
		seedName: String,
		seedPhrase: String,
		navigator: Navigator,
	) {
		viewModelScope.launch {
			val repository = ServiceLocator.activityScope!!.seedRepository
			repository.addSeed(
				seedName = seedName,
				seedPhrase = seedPhrase,
				navigator = navigator,
				isOptionalAuth = false
			)
		}
	}

	fun onTextEntry(newText: String) {
		val uniffiInteractor = ServiceLocator.uniffiInteractor
		val result =
			runBlocking { uniffiInteractor.navigate(Action.TEXT_ENTRY, newText) }
		when (result) {
			is OperationResult.Err -> {
				submitErrorState("error in restore text entry $result")
			}

			is OperationResult.Ok -> {
				val screenData =
					result.result.screenData as? ScreenData.RecoverSeedPhrase ?: let {
						submitErrorState("wrong state keyset restore text entry $result")
						return
					}
				_recoverState.value = screenData.f.toKeysetRecoverModel()
			}
		}
	}

	fun addWord(word: String) {
		val uniffiInteractor = ServiceLocator.uniffiInteractor
		val result =
			runBlocking { uniffiInteractor.navigate(Action.PUSH_WORD, word) }
		when (result) {
			is OperationResult.Err -> {
				submitErrorState("error in add suggestion word $result")
			}

			is OperationResult.Ok -> {
				val screenData =
					result.result.screenData as? ScreenData.RecoverSeedPhrase ?: let {
						submitErrorState("wrong state in add suggestion word $result")
						return
					}
				_recoverState.value = screenData.f.toKeysetRecoverModel()
			}
		}
	}
}
