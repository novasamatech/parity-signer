package io.parity.signer.screens.keysets.restore

import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.backend.RecoverSeedInteractor
import io.parity.signer.domain.backend.mapError
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.runBlocking

class KeysetRecoverViewModel : ViewModel() {

	private val backendInteractor = RecoverSeedInteractor()

	private val _recoverSeed = MutableStateFlow<KeysetRecoverModel>(
		KeysetRecoverModel.new(getGuessWords(""))
	)
	val recoverSeed = _recoverSeed.asStateFlow()

	val existingSeeds = ServiceLocator.seedStorage.lastKnownSeedNames

	private fun getGuessWords(input: String): List<String> {
		return runBlocking {
			backendInteractor.seedPhraseGuessWords(input).mapError() ?: emptyList()
		}
	}

	private fun validateSeedPhrase(phrase: List<String>): Boolean {
		return runBlocking {
			backendInteractor.validateSeedPhrase(phrase.joinToString { " " })
				.mapError() ?: false
		}
	}

	fun onUserInput(rawUserInput: String) {
		if (rawUserInput.isEmpty()) {
			_recoverSeed.value = _recoverSeed.value.let { model ->
				model.copy(
					rawUserInput = KeysetRecoverModel.SPACE_CHARACTER.toString(),
					enteredWords = model.enteredWords.dropLast(1)
				)
			}
		} else if (rawUserInput.first() != KeysetRecoverModel.SPACE_CHARACTER) {
			//user removed first symbol?
			_recoverSeed.value = _recoverSeed.value.let { model ->
				model.copy(
					rawUserInput = KeysetRecoverModel.SPACE_CHARACTER.toString() + rawUserInput,
				)
			}
		} else {
			val input = rawUserInput.drop(1)
//todo dmitry

			val guessing = getGuessWords(input)
			when (guessing.size) {
				1 ->
			}
		}
	}

	//	todo dmitry
//	rust/db_handling/src/interface_signer.rs:805
//	ios/PolkadotVault/Screens/CreateKey/RecoverKeySet/RecoverKeySetSeedPhraseView.swift:200
	fun onAddword(word: String) {
		_recoverSeed.value = _recoverSeed.value.let { model: KeysetRecoverModel ->
			val newDraft = model.enteredWords + word
			model.copy(
				rawUserInput = KeysetRecoverModel.SPACE_CHARACTER.toString(),
				enteredWords = newDraft,
				validSeed = validateSeedPhrase(newDraft),
				suggestedWords = getGuessWords("")
			)
		}
	}
}


data class KeysetRecoverModel(
	val rawUserInput: String,
	val suggestedWords: List<String>,
	val enteredWords: List<String>,
	val validSeed: Boolean
) {
	companion object {

		const val SPACE_CHARACTER: Char = ' '
		const val NEW_LINE: Char = '\n'

		fun new(suggestedWords: List<String>): KeysetRecoverModel {
			return KeysetRecoverModel(
				rawUserInput = SPACE_CHARACTER.toString(),
				suggestedWords = suggestedWords,
				enteredWords = emptyList(),
				validSeed = false,
			)
		}

		fun stub(): KeysetRecoverModel {
			return KeysetRecoverModel(
				rawUserInput = "ggf",
				suggestedWords = listOf("ggfhg", "goha"),
				enteredWords = listOf("somve", "words", "that", "are", "draft"),
				validSeed = false,
			)
		}
	}
}
