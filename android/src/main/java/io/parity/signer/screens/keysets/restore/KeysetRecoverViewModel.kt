package io.parity.signer.screens.keysets.restore

import androidx.lifecycle.ViewModel
import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.backend.RecoverSeedInteractor
import io.parity.signer.domain.backend.UniffiResult
import io.parity.signer.domain.backend.mapError
import io.parity.signer.domain.submitErrorState
import io.parity.signer.uniffi.ScreenData
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.runBlocking

class KeysetRecoverViewModel : ViewModel() {

	private val backendInteractor = RecoverSeedInteractor()

	private val _recoverSeed = MutableStateFlow<KeysetRecoverModel>(
		KeysetRecoverModel.new(
			runBlocking {
				backendInteractor.seedPhraseGuessWords("").mapError() ?: emptyList()
			}
		)
	)
	val recoverSeed = _recoverSeed.asStateFlow()

	val existingSeeds = ServiceLocator.seedStorage.lastKnownSeedNames

 	fun onTextEntry(userInput: String) {
		val result = runBlocking {
			backendInteractor.seedPhraseGuessWords(userInput)
		}
		when (result) {
			is UniffiResult.Error -> {
				submitErrorState("error in restore text entry $result")
			}

			is UniffiResult.Success -> {
//				val screenData =
//					result.result.screenData as? ScreenData.RecoverSeedPhrase ?: let {
//						submitErrorState("wrong state keyset restore text entry $result")
//						return
//					}
//				_recoverState.value = screenData.f.toKeysetRecoverModel()
			}
		}
	}

	//	todo dmitry
//	rust/db_handling/src/interface_signer.rs:805
//	ios/PolkadotVault/Screens/CreateKey/RecoverKeySet/RecoverKeySetSeedPhraseView.swift:200
	fun addWord(word: String) {
		val result =
			runBlocking { backendInteractor.validateSeedPhrase() }
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
				_recoverSeed.value = screenData.f.toKeysetRecoverModel()
			}
		}
	}
}
