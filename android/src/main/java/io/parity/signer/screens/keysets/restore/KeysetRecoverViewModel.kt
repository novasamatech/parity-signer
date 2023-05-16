package io.parity.signer.screens.keysets.restore

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.Navigator
import kotlinx.coroutines.launch


class KeysetRecoverViewModel: ViewModel() {



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

	}
}
