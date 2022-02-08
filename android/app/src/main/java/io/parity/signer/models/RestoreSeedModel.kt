package io.parity.signer.models

import android.util.Log
import androidx.compose.runtime.*
import androidx.compose.ui.text.TextRange
import androidx.compose.ui.text.input.TextFieldValue

//TODO: Move logic to Rust!
/**
 * Seed restore tool logic
 *
 * This should not be ViewModel though;
 * Ideally all ViewModels should not hold persistent secrets
 * as they are prone to silent memory leaks
 */
class RestoreSeedModel(
	var seedPhrase: MutableState<List<String>>,
	var seedWord: MutableState<TextFieldValue>,
	var guessWord: MutableState<List<String>>,
	var seedValid: MutableState<Boolean>,
	val guess: (word: String) -> List<String>,
	val check: (seedPhrase: String) -> String?
) {

	/**
	 * This is all that's needed: get a word in user input,
	 * return proper input state
	 */
	fun update(word: String) {
		if (word == "") {
			if (seedPhrase.value.count() > 0) {
				seedPhrase.value = seedPhrase.value.subList(0, seedPhrase.value.lastIndex)
			}
			guessWord.value = guess("")
			seedWord.value = TextFieldValue(" ", selection = TextRange(1))
		} else {
			if (word.first() != ' ') {
				if (seedPhrase.value.count() > 0) {
					seedPhrase.value = seedPhrase.value.subList(0, seedPhrase.value.lastIndex)
				}
				guessWord.value = guess("")
				seedWord.value = TextFieldValue(" ", selection = TextRange(1))
			} else {
				if (word.last() == ' ') {
					val tempword = word.dropLast(1)
					if (guessWord.value.count() == 1) {
						seedPhrase.value += guessWord.value.last()
						guessWord.value = guess("")
						seedWord.value = TextFieldValue(" ", selection = TextRange(1))
					} else if (guessWord.value.isEmpty()) {
						guessWord.value = guess(tempword)
						seedWord.value = TextFieldValue(tempword, selection = TextRange(tempword.length))
					} else {
						if (guessWord.value.contains(tempword.drop(1))) {
							seedPhrase.value += tempword.drop(1)
							guessWord.value = guess("")
							seedWord.value = TextFieldValue(" ", selection = TextRange(1))
						}
					}
				} else {
					guessWord.value = guess(word.substring(1))
					seedWord.value = TextFieldValue(word, selection = TextRange(word.length))
				}
			}
		}
		seedValid.value = check(seedPhrase.value.joinToString(" ")).isNullOrBlank()
		//return TextFieldValue(word, selection = TextRange(word.length))
	}

	/**
	 * Select word and send it to seed phrase collector
	 */
	fun select(word: String) {
		seedPhrase.value += word
		guessWord.value = guess("")
		seedWord.value = TextFieldValue(" ", selection = TextRange(1))
	}
}
