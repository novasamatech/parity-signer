package io.parity.signer.domain

import android.content.*
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

class SignerDataModel : ViewModel() {

	// todo migrate to use dependencies from ServiceLocator rather than expecting them here
	val context: Context get() = ServiceLocator.appContext
	val activity: FragmentActivity get() = ServiceLocator.activityScope!!.activity

	val navigator by lazy { SignerNavigator(this) }

	// Current key details, after rust API will migrate to REST-like should not store this value here.
	internal var lastOpenedKeyDetails: MKeyDetails? = null

	// Transaction
	internal var action = JSONObject()

	val seedStorage: SeedStorage = ServiceLocator.seedStorage
	private val databaseAssetsInteractor = DatabaseAssetsInteractor(context, seedStorage)
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

	// Data storage locations
	internal var dbName: String = ""

	// Observables for screens state
	val authenticated: StateFlow<Boolean> = ServiceLocator.authentication.auth

	val networkState: StateFlow<NetworkState> = networkExposedStateKeeper.networkState

	val actionResult: StateFlow<ActionResult> = _actionResult

	val localNavAction: StateFlow<LocalNavAction> = _localNavAction

	// MARK: init boilerplate begin

	/**
	 * Don't forget to call real init after defining context!
	 */
	fun lateInit() {
		// Define local database name
		dbName = context.getDbNameFromContext()

		// Imitate ios behavior
		val authentication = ServiceLocator.authentication
		authentication.authenticate(activity) {
			seedStorage.init(context)
			totalRefresh()
		}
	}

	/**
	 * todo onboarding remove this
	 * Populate database!
	 * This is normal onboarding
	 */
	fun onBoard() {
		databaseAssetsInteractor.wipe()
		databaseAssetsInteractor.copyAsset("")
		totalRefresh()
		historyInitHistoryWithCert()
	}

	/**
	 * Init database with no general certificate
	 */
	private fun wipeDbNoCert() {
		databaseAssetsInteractor.wipe()
		databaseAssetsInteractor.copyAsset("")
		totalRefresh()
		historyInitHistoryNoCert()
	}

	// MARK: Init boilerplate end

	// MARK: General utils begin

	/**
	 * This returns the app into starting state; should be called
	 * on all "back"-like events and new screen spawns just in case
	 */
	fun totalRefresh() {
		val checkRefresh = context.isDbCreatedAndOnboardingPassed()
		if (checkRefresh) totalRefreshDbExist() else totalRefreshDbMissing()
	}

	private fun totalRefreshDbExist() {
		_onBoardingDone.value = OnboardingWasShown.Yes
		val allNames = seedStorage.getSeedNames()
		initNavigation(dbName, allNames.toList())
		updateAlertState()
		navigator.navigate(Action.START)
	}

	private fun totalRefreshDbMissing() {
		_onBoardingDone.value = OnboardingWasShown.No
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


