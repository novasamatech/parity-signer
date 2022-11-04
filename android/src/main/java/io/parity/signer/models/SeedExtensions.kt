package io.parity.signer.models

import android.util.Log
import android.widget.Toast
import io.parity.signer.components.SeedBoxStatus
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.historySeedNameWasShown
import io.parity.signer.uniffi.initNavigation
import io.parity.signer.uniffi.updateSeedNames
import kotlinx.coroutines.suspendCancellableCoroutine


suspend fun SignerDataModel.getSeedPhraseForBackup(seedName: String,
): String {
		authentication.authenticate(activity) { //todo dmitry do suspend function
			val seedPhrase = getSeed(seedName, backup = true)
			return seedPhrase
		}
	suspendCancellableCoroutine<> {  }
}
