package io.parity.signer.domain

import android.content.*
import android.security.keystore.UserNotAuthenticatedException
import androidx.fragment.app.FragmentActivity
import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.storage.DatabaseAssetsInteractor
import io.parity.signer.domain.storage.SeedStorage
import io.parity.signer.screens.onboarding.OnboardingWasShown
import io.parity.signer.uniffi.*
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import org.json.JSONObject

class MainFlowViewModel : ViewModel() {

	// todo migrate to use dependencies from ServiceLocator rather than expecting them here
	val context: Context get() = ServiceLocator.appContext
	val activity: FragmentActivity get() = ServiceLocator.activityScope!!.activity

	val navigator by lazy { SignerNavigator(this) }

	// Current key details, after rust API will migrate to REST-like should not store this value here.
	internal var lastOpenedKeyDetails: MKeyDetails? = null

	// Transaction
	internal var action = JSONObject()

	val seedStorage: SeedStorage = ServiceLocator.seedStorage
	private val databaseAssetsInteractor: DatabaseAssetsInteractor = ServiceLocator.databaseAssetsInteractor
	private val networkExposedStateKeeper = NetworkExposedStateKeeper(context)

	// Navigator
	internal val _actionResult = MutableStateFlow(
		ActionResult(
			screenLabel = "",
			back = false,
			footer = false,
			footerButton = null,
			rightButton = null,
			screenNameType = ScreenNameType.H4,
			screenData = ScreenData.Documents,
			modalData = null,
			alertData = null,
		)
	)

	internal val _localNavAction = MutableStateFlow<LocalNavAction>(
		LocalNavAction.None
	)

	// Observables for screens state
	val networkState: StateFlow<NetworkState> = networkExposedStateKeeper.networkState

	val actionResult: StateFlow<ActionResult> = _actionResult

	val localNavAction: StateFlow<LocalNavAction> = _localNavAction

	// MARK: init boilerplate begin

	/**
	 * Don't forget to call real init after defining context!
	 */
	fun lateInit() {
		// Imitate ios behavior
		val authentication = ServiceLocator.authentication
		authentication.authenticate(activity) {
			seedStorage.init(context)
			totalRefresh()
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

	// MARK: General utils begin

	/**
	 * This returns the app into starting state; should be called
	 * on all "back"-like events and new screen spawns just in case
	 */
	fun totalRefresh() {
		val checkRefresh = context.isDbCreatedAndOnboardingPassed()
		if (checkRefresh) totalRefreshDbExist()
	}

	private fun totalRefreshDbExist() {
		val allNames = seedStorage.getSeedNames()
		initNavigation(context.getDbNameFromContext(), allNames.toList())
		networkExposedStateKeeper.updateAlertState()
		navigator.navigate(Action.START)
	}

	/**
	 * Auth user and wipe the Signer to initial state
	 */
	fun wipeToFactory() {
		val authentication = ServiceLocator.authentication
		authentication.authenticate(activity) {
			databaseAssetsInteractor.wipe()
			totalRefresh()
		}
	}

	/**
	 * Auth user and wipe Signer to state without general verifier certificate
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


