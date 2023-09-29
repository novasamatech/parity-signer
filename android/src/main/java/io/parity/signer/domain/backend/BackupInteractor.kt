package io.parity.signer.domain.backend

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext


class BackupInteractor {

	suspend fun notifyRustSeedWasShown(seedName: String) {
		withContext(Dispatchers.IO) {
			io.parity.signer.uniffi.historySeedWasShown(seedName)
		}
	}
}
