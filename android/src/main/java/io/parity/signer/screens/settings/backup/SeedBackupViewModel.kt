package io.parity.signer.screens.settings.backup

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.getSeedPhraseForBackup
import kotlinx.coroutines.runBlocking


internal class SeedBackupViewModel() : ViewModel() {

	val seedStorage = ServiceLocator.seedStorage

	fun getSeeds(): List<String> {
		return seedStorage.getSeedNames().toList()
	}

	suspend fun getSeedPhrase(seedName: String): String? {
		return getSeedPhraseForBackup(seedName)
	}
}
