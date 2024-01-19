package io.parity.signer

import android.app.Application
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.submitErrorState
import io.parity.signer.uniffi.ErrorDisplayed
import io.parity.signer.uniffi.initLogging
import timber.log.Timber
import timber.log.Timber.*
import java.lang.Thread.UncaughtExceptionHandler


class PolkadotVaultApp : Application() {
	override fun onCreate() {
		super.onCreate()
		// actually load RustNative code
		System.loadLibrary("signer")

		initLogging("SIGNER_RUST_LOG")

		if (BuildConfig.DEBUG) {
			Timber.plant(DebugTree())
		}

		val defaultHandler = Thread.getDefaultUncaughtExceptionHandler()
		Thread.setDefaultUncaughtExceptionHandler(
			RootExceptionHandler(defaultHandler)
		)

		ServiceLocator.initAppDependencies(this)
	}
}


class RootExceptionHandler(
	private val defaultHandler: UncaughtExceptionHandler?
) : UncaughtExceptionHandler {
	private val TAG = "SignerExceptionHandler"

	override fun uncaughtException(t: Thread, e: Throwable) {
		val rustStr = findErrorDisplayedStr(e)
		if (rustStr != null) {
			Timber.e(TAG, "Rust caused ErrorDisplay message was: ${rustStr.s}")
			submitErrorState("rust error not handled, fix it!")
		} else {
			defaultHandler?.uncaughtException(t, e) ?: throw e
		}
	}

	private tailrec fun findErrorDisplayedStr(exception: Throwable): ErrorDisplayed.Str? {
		if (exception is ErrorDisplayed.Str) {
			return exception
		}
		val cause = exception.cause
		return if (cause != null) {
			findErrorDisplayedStr(cause)
		} else {
			null
		}
	}
}
