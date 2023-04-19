package io.parity.signer.domain

import android.util.Log
import io.parity.signer.BuildConfig
import io.parity.signer.uniffi.ErrorDisplayed
import kotlinx.coroutines.Dispatchers
import java.lang.RuntimeException
import java.util.concurrent.Executors

fun submitErrorState(message: String) {
	Log.e("error state", message)
	if (BuildConfig.DEBUG) {
		throw RuntimeException(message)
	}
}


fun ErrorDisplayed.getDetailedDescriptionString(): String {
	return this.javaClass.name + "Message: " + message
}

