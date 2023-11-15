package io.parity.signer.screens.settings.backup

import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.storage.mapError


internal class SeedBackupViewModel() : ViewModel() {

	private val seedStorage = ServiceLocator.seedStorage
	private val seedRepository = ServiceLocator.activityScope!!.seedRepository

	fun getSeeds(): List<String> {
		return seedStorage.getSeedNames().toList()
	}

	suspend fun getSeedPhrase(seedName: String): String? {
		return seedRepository.getSeedPhraseForceAuth(seedName).mapError()
	}
}
