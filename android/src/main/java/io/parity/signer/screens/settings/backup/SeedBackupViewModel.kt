package io.parity.signer.screens.settings.backup

import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator


internal class SeedBackupViewModel() : ViewModel() {

	val seedStorage = ServiceLocator.seedStorage

	fun getSeeds(): List<String> {
		return seedStorage.getSeedNames().toList()
	}


}
