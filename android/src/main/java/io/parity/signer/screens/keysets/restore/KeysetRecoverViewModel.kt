package io.parity.signer.screens.keysets.restore

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.Navigator
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch


class KeysetRecoverViewModel: ViewModel() {

	private val _recoverState = MutableStateFlow<KeysetRecoverModel?>(null)
	val recoverState = _recoverState.asStateFlow()

	fun initValue(model: KeysetRecoverModel) {
		_recoverState.value = model
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
//todo dmitry
//		button(Action.TEXT_ENTRY, it.text)
	}

	fun addWord(word: String) {
//todo dmitry
	}
}
