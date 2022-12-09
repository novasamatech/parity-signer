package io.parity.signer.dependencygraph

import android.content.Context
import io.parity.signer.backend.UniffiInteractor
import io.parity.signer.models.Authentication

object ServiceLocator {

	@Volatile private var _backendLocator: BackendLocator? = null
	val backendLocator: BackendLocator
		get() = _backendLocator
			?: throw RuntimeException("dependency is not initialized yet")


	fun initBackendDeps(context: Context) {
		_backendLocator = BackendLocator(context.getDbNameFromContext())
	}

	val authentication by lazy { Authentication() }
}

fun Context.getDbNameFromContext() =
	applicationContext.filesDir.toString() + "/Database"


class BackendLocator(dbname: String) {
	val uniffiInteractor by lazy { UniffiInteractor(dbname) }
}
