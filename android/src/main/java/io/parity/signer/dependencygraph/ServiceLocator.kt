package io.parity.signer.dependencygraph

import android.content.Context
import androidx.fragment.app.FragmentActivity
import io.parity.signer.backend.UniffiInteractor
import io.parity.signer.domain.Authentication
import io.parity.signer.domain.getDbNameFromContext
import io.parity.signer.domain.storage.SeedRepository
import io.parity.signer.domain.storage.SeedStorage

object ServiceLocator {

	lateinit var appContext: Context

	fun initAppDependencies(appContext: Context) {
		this.appContext = appContext
		_backendScope = BackendScope(appContext.getDbNameFromContext())
	}

	fun initActivityDependencies(activity: FragmentActivity) {
		_activityScope = ActivityScope(activity)
	}

	fun deinitActivityDependencies() {
		_activityScope = null
	}

	private var _backendScope: BackendScope? = null
	val backendScope: BackendScope
		get() = _backendScope
			?: throw RuntimeException("dependency is not initialized yet")

	@Volatile private var _activityScope: ActivityScope? = null
	val activityScope: ActivityScope? get() = _activityScope

	val seedStorage: SeedStorage = SeedStorage()
	val authentication = Authentication()


	class BackendScope(dbname: String) {
		val uniffiInteractor by lazy { UniffiInteractor(dbname) }
	}

	class ActivityScope(val activity: FragmentActivity) {
		val seedRepository: SeedRepository = SeedRepository(seedStorage,
			authentication, activity)
	}
}

