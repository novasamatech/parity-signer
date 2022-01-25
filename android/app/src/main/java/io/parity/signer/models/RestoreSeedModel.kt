package io.parity.signer.models

import androidx.compose.runtime.getValue
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.text.TextRange
import androidx.compose.ui.text.input.TextFieldValue
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel

//TODO: Move logic here?
/**
 * Seed restore tool logic
 */
class RestoreSeedModel {
	private var _seedPhrase = MutableLiveData(mutableListOf<String>())
	private var _guessWord = MutableLiveData(listOf<String>())
	private var _seedValid = MutableLiveData(false)

	val seedPhrase: LiveData<MutableList<String>> = _seedPhrase
	val guessWord: LiveData<List<String>> = _guessWord
	val seedValid: LiveData<Boolean> = _seedValid

	fun update(seedWord: String): TextFieldValue {
		var text: String = seedWord



		return TextFieldValue(text, selection = TextRange(text.length))
	}
}
