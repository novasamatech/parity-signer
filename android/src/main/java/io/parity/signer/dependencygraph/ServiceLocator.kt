package io.parity.signer.dependencygraph

import android.content.Context
import androidx.fragment.app.FragmentActivity
import io.parity.signer.domain.backend.UniffiInteractor
import io.parity.signer.components.networkicon.UnknownNetworkColorsGenerator
import io.parity.signer.domain.Authentication
import io.parity.signer.domain.NetworkExposedStateKeeper
import io.parity.signer.domain.storage.BananaSplitRepository
import io.parity.signer.domain.storage.ClearCryptedStorage
import io.parity.signer.domain.storage.DatabaseAssetsInteractor
import io.parity.signer.domain.storage.PreferencesRepository
import io.parity.signer.domain.storage.SeedRepository
import io.parity.signer.domain.storage.SeedStorage

object ServiceLocator {

	lateinit var appContext: Context

	fun initAppDependencies(appContext: Context) {
		this.appContext = appContext
	}

	fun initActivityDependencies(activity: FragmentActivity) {
		_activityScope = ActivityScope(activity)
	}

	fun deinitActivityDependencies() {
		_activityScope = null
	}

	@Volatile
	private var _activityScope: ActivityScope? = null
	val activityScope: ActivityScope? get() = _activityScope

	val uniffiInteractor by lazy { UniffiInteractor(appContext) }

	val seedStorage: SeedStorage = SeedStorage()
	val clearCryptedStorage: ClearCryptedStorage = ClearCryptedStorage()
	val preferencesRepository: PreferencesRepository by lazy {
		PreferencesRepository(
			appContext
		)
	}
	val databaseAssetsInteractor by lazy {
		DatabaseAssetsInteractor(
			appContext,
			seedStorage
		)
	}
	val networkExposedStateKeeper by lazy {
		NetworkExposedStateKeeper(
			appContext,
			uniffiInteractor
		)
	}
	val authentication = Authentication()
	val unknownNetworkColorsGenerator: UnknownNetworkColorsGenerator =
		UnknownNetworkColorsGenerator()

	class ActivityScope(val activity: FragmentActivity) {
		val seedRepository: SeedRepository = SeedRepository(
			seedStorage = seedStorage,
			clearCryptedStorage = clearCryptedStorage,
			authentication = authentication,
			activity = activity,
			uniffiInteractor = uniffiInteractor,
		)

		val bsRepository: BananaSplitRepository = BananaSplitRepository(
			seedStorage = seedStorage,
			clearCryptedStorage = clearCryptedStorage,
			authentication = authentication,
			activity = activity,
			uniffiInteractor = uniffiInteractor,
		)
	}
}

