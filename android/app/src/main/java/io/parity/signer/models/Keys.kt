package io.parity.signer.models

import android.util.Log
import androidx.compose.runtime.MutableState
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.Address
import io.parity.signer.uniffi.DerivationDestination
import io.parity.signer.uniffi.DerivationCheck as DerivationCheckFFI

/**
 * Add key to database; uses phone crypto to fetch seeds!
 */
fun SignerDataModel.addKey(path: String, seedName: String) {
	authentication.authenticate(activity) {
		try {
			val seedPhrase = getSeed(seedName)
			if (seedPhrase.isNotBlank()) {
				pushButton(Action.GO_FORWARD, path, seedPhrase)
			}
		} catch (e: java.lang.Exception) {
			Log.e("Add key error", e.toString())
			_lastError.value = e.toString()
		}
	}
}

/**
 * Checker for derivation path
 */
class DerivationCheck(
	var buttonGood: MutableState<Boolean>,
	var whereTo: MutableState<DerivationDestination?>,
	private var collision: MutableState<Address?>,
	var checkCallback: (path: String) -> Unit
) {
	/**
	 * Call to perform on every path change
	 */
	fun check(path: String) {
		val checkResult = checkCallback(path)
		Log.d("checkResult", "$checkResult")
	}

	fun fromFFI(derivationCheck: DerivationCheckFFI) {
		buttonGood.value = derivationCheck.buttonGood
		whereTo.value = derivationCheck.whereTo
		collision.value = derivationCheck.collision
		derivationCheck.error?.let {
			Log.d("collision checker error", it)
		}
	}
}

fun SignerDataModel.increment(number: Int, seedName: String) {
	authentication.authenticate(activity) {
		try {
			val seedPhrase = getSeed(seedName)
			if (seedPhrase.isNotBlank()) {
				pushButton(Action.INCREMENT, number.toString())
			}
		} catch (e: java.lang.Exception) {
			Log.e("Add key error", e.toString())
			_lastError.value = e.toString()
		}
	}
}
