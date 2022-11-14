package io.parity.signer

import android.app.Application
import android.util.Log
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.uniffi.ErrorDisplayed
import java.lang.Thread.UncaughtExceptionHandler

class SignerApp : Application() {
	override fun onCreate() {
		super.onCreate()
		Thread.setDefaultUncaughtExceptionHandler(SignerExceptionHandler())
		ServiceLocator.initBackendDeps(this)
	}
}


class SignerExceptionHandler : UncaughtExceptionHandler {
	private val TAG = "SignerExceptionHandler"

	override tailrec fun uncaughtException(t: Thread, e: Throwable) {
		if (e is ErrorDisplayed.Str) {
			Log.e(TAG, "Rust caused ErrorDisplay message was: ${e.s}")
		}
		val cause = e.cause
		if (cause != null) {
			uncaughtException(Thread.currentThread(), cause)
		}
	}
}
