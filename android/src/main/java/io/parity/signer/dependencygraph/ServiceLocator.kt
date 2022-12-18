package io.parity.signer.dependencygraph

import android.content.Context
import io.parity.signer.backend.UniffiInteractor
import io.parity.signer.models.Authentication
import io.parity.signer.models.storage.SeedStorage

object ServiceLocator {

	fun initAppDependencies(appContext: Context) {
		_backendLocator = BackendLocator(appContext.getDbNameFromContext())
	}

	private var _backendLocator: BackendLocator? = null
	val backendLocator: BackendLocator
		get() = _backendLocator
			?: throw RuntimeException("dependency is not initialized yet")

	val seedStorage: SeedStorage = SeedStorage()

	val authentication by lazy { Authentication() }
}

fun Context.getDbNameFromContext() =
	applicationContext.filesDir.toString() + "/Database"


class BackendLocator(dbname: String) {
	val uniffiInteractor by lazy { UniffiInteractor(dbname) }
}
