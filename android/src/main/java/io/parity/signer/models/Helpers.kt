package io.parity.signer.models

import android.util.Log
import io.parity.signer.BuildConfig
import java.lang.RuntimeException

fun submitErrorState(message: String) {
	if (BuildConfig.DEBUG) {
		throw RuntimeException(message)
	} else {
		Log.e("error state", message)
	}
}
