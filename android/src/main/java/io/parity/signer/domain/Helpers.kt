package io.parity.signer.domain

import android.util.Log
import io.parity.signer.BuildConfig
import io.parity.signer.uniffi.ErrorDisplayed
import java.lang.RuntimeException

fun submitErrorState(message: String) {
	Log.e("error state", message)
	if (BuildConfig.DEBUG) {
		throw RuntimeException(message)
	}
}


fun ErrorDisplayed.getDebugDetailedDescriptionString(): String {
	return this.javaClass.name + "Message: " + message
}

