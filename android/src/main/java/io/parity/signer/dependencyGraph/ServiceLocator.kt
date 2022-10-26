package io.parity.signer.dependencyGraph

import android.content.Context
import io.parity.signer.backend.UniffiInteractor

object ServiceLocator {

	@Volatile private var _backendLocator: BackendLocator? = null
	val backendLocator: BackendLocator
		get() = _backendLocator
			?: throw RuntimeException("dependency is not initialized yet")


	fun initBackendDeps(context: Context) {
		_backendLocator = BackendLocator(context.getDbNameFromContext())
	}
}

fun Context.getDbNameFromContext() =
	applicationContext.filesDir.toString() + "/Database"


class BackendLocator(dbname: String) {
	val uniffiInteractor by lazy { UniffiInteractor(dbname) }
}
