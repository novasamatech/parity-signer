package io.parity.signer.domain.usecases

import android.security.keystore.UserNotAuthenticatedException
import androidx.fragment.app.FragmentActivity
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.Callback
import io.parity.signer.domain.getDbNameFromContext
import io.parity.signer.domain.isDbCreatedAndOnboardingPassed
import io.parity.signer.domain.storage.DatabaseAssetsInteractor
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.historyInitHistoryNoCert
import io.parity.signer.uniffi.historyInitHistoryWithCert
import io.parity.signer.uniffi.initNavigation


class ResetUseCase {

	private val seedStorage = ServiceLocator.seedStorage
	private val databaseAssetsInteractor: DatabaseAssetsInteractor =
		ServiceLocator.databaseAssetsInteractor
	private val appContext = ServiceLocator.appContext
	private val networkExposedStateKeeper =
		ServiceLocator.networkExposedStateKeeper
	private val navigator = ServiceLocator.navigator
	private val activity: FragmentActivity
		get() = ServiceLocator.activityScope!!.activity
	//todo as in SharevView model - remove from here?

	//todo dmitry test
	fun wipeToFactory(onAfterWide: Callback) {
		val authentication = ServiceLocator.authentication
		authentication.authenticate(activity) {
			databaseAssetsInteractor.wipe()
			totalRefresh()
			onAfterWide()
		}
	}
	private fun totalRefreshDbExist() {
		val allNames = seedStorage.getSeedNames()
		initNavigation(appContext.getDbNameFromContext(), allNames.toList())
		ServiceLocator.uniffiInteractor.wasRustInitialized.value = true
		networkExposedStateKeeper.updateAlertStateFromHistory()
		navigator.navigate(Action.START)
	}

	/**
	 * Populate database!
	 * This is first start of the app
	 */
	private fun initAssetsAndTotalRefresh() {
		databaseAssetsInteractor.wipe()
		databaseAssetsInteractor.copyAsset("")
		totalRefreshDbExist()
		historyInitHistoryWithCert()
	}

	/**
	 * Auth user and wipe Vault to state without general verifier certificate
	 */
	fun wipeToJailbreak() {
		val authentication = ServiceLocator.authentication
		authentication.authenticate(activity) {
			wipeDbNoCert()
		}
	}

	/**
	 * Init database with no general certificate
	 * @throws UserNotAuthenticatedException
	 */
	private fun wipeDbNoCert() {
		databaseAssetsInteractor.wipe()
		databaseAssetsInteractor.copyAsset("")
		totalRefresh()
		historyInitHistoryNoCert()
	}

	/**
	 * This returns the app into starting state;
	 */
	fun totalRefresh() {
		if (!seedStorage.isInitialized()) {
			seedStorage.init(appContext)
		}
		if (!appContext.isDbCreatedAndOnboardingPassed()) {
			initAssetsAndTotalRefresh()
		} else {
			totalRefreshDbExist()
		}
	}
}
