package io.parity.signer.domain

import android.annotation.SuppressLint
import android.content.*
import android.security.keystore.UserNotAuthenticatedException
import androidx.fragment.app.FragmentActivity
import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.storage.DatabaseAssetsInteractor
import io.parity.signer.domain.storage.SeedStorage
import io.parity.signer.uniffi.*
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import org.json.JSONObject


@SuppressLint("StaticFieldLeak")
class SharedViewModel() : ViewModel() {
	val context: Context = ServiceLocator.appContext.applicationContext
	val activity: FragmentActivity = ServiceLocator.activityScope!!.activity

	init {
		// Imitate ios behavior
		val authentication = ServiceLocator.authentication
		authentication.authenticate(activity) {
			totalRefresh()
		}
	}
	val navigator by lazy { SignerNavigator(this) }

	// Current key details, after rust API will migrate to REST-like should not store this value here.
	internal var lastOpenedKeyDetails: MKeyDetails? = null

	// Transaction
	internal var action = JSONObject()

	val seedStorage: SeedStorage = ServiceLocator.seedStorage
	private val databaseAssetsInteractor: DatabaseAssetsInteractor =
		ServiceLocator.databaseAssetsInteractor
	private val networkExposedStateKeeper =
		ServiceLocator.networkExposedStateKeeper

	// Navigator
	internal val _actionResult = MutableStateFlow<ActionResult?>(null
	)

	internal val _localNavAction = MutableStateFlow<LocalNavAction>(
		LocalNavAction.None
	)

	// Observables for screens state
	val networkState: StateFlow<NetworkState> =
		networkExposedStateKeeper.airplaneModeState
	val actionResult: StateFlow<ActionResult?> = _actionResult
	val localNavAction: StateFlow<LocalNavAction> = _localNavAction
	val authenticated: StateFlow<Boolean> = ServiceLocator.authentication.auth

	// MARK: init boilerplate begin

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

	// MARK: General utils begin

	/**
	 * This returns the app into starting state;
	 */
	fun totalRefresh() {
		if (!seedStorage.isInitialized()) {
			seedStorage.init(context)
		}
		if (!context.isDbCreatedAndOnboardingPassed()) {
			initAssetsAndTotalRefresh()
		} else {
			totalRefreshDbExist()
		}
	}

	private fun totalRefreshDbExist() {
		val allNames = seedStorage.getSeedNames()
		initNavigation(context.getDbNameFromContext(), allNames.toList())
		ServiceLocator.uniffiInteractor.wasRustInitialized.value = true
		networkExposedStateKeeper.updateAlertStateFromHistory()
		navigator.navigate(Action.START)
		if (allNames.isEmpty()) {
			//workaround to hide create new bottom sheet while #1618 is not merged
			//https://github.com/paritytech/parity-signer/pull/1618
			navigator.navigate(Action.GO_BACK)
		}
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
	 * Auth user and wipe the Vault to initial state
	 */
	fun wipeToFactory() {
		val authentication = ServiceLocator.authentication
		authentication.authenticate(activity) {
			databaseAssetsInteractor.wipe()
			totalRefresh()
		}
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

	fun getAppVersion(): String {
		return context.packageManager.getPackageInfo(
			context.packageName,
			0
		).versionName
	}

	fun acknowledgeWarning() {
		networkExposedStateKeeper.acknowledgeWarning()
	}
}

