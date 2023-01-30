package io.parity.signer.domain

import android.util.Log
import io.parity.signer.BuildConfig
import java.lang.RuntimeException

fun submitErrorState(message: String) {
	Log.e("error state", message)
	if (BuildConfig.DEBUG) {
		throw RuntimeException(message)
	}
}
