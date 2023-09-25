package io.parity.signer.domain.backend

import kotlinx.coroutines.GlobalScope
import kotlinx.coroutines.launch


class BackupInteractor {

	fun notifyRustSeedWasShown(seedName: String) {
		GlobalScope.launch {
			io.parity.signer.uniffi.historySeedWasShown(seedName)
		}
	}
}
