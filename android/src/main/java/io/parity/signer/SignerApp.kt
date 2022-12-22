package io.parity.signer

import android.app.Application
import android.util.Log
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.models.submitErrorState
import io.parity.signer.uniffi.ErrorDisplayed
import io.parity.signer.uniffi.initLogging
import java.lang.Thread.UncaughtExceptionHandler

class SignerApp : Application() {
	override fun onCreate() {
		super.onCreate()
		// actually load RustNative code
		System.loadLibrary("signer")

		initLogging("SIGNER_RUST_LOG")

		val defaultHandler = Thread.getDefaultUncaughtExceptionHandler()
		Thread.setDefaultUncaughtExceptionHandler(
			SignerExceptionHandler(defaultHandler)
		)

		ServiceLocator.initAppDependencies(this)
	}
}


class SignerExceptionHandler(private val defaultHandler: UncaughtExceptionHandler?) :
	UncaughtExceptionHandler {
	private val TAG = "SignerExceptionHandler"

	override fun uncaughtException(t: Thread, e: Throwable) {
		val rustStr = findErrorDisplayed(e)
		if (rustStr != null) {
			Log.e(TAG, "Rust caused ErrorDisplay message was: ${rustStr.s}")
			submitErrorState("rust error not handled, fix it!")
		} else {
			defaultHandler?.uncaughtException(t, e) ?: throw e
		}
	}

	private tailrec fun findErrorDisplayed(exception: Throwable): ErrorDisplayed.Str? {
		if (exception is ErrorDisplayed.Str) {
			return exception
		}
		val cause = exception.cause
		return if (cause != null) {
			findErrorDisplayed(cause)
		} else {
			null
		}
	}
}
