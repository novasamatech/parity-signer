package io.parity.signer.models

import android.annotation.SuppressLint
import android.content.*
import android.provider.Settings
import androidx.fragment.app.FragmentActivity
import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.dependencygraph.getDbNameFromContext
import io.parity.signer.models.storage.SeedStorage
import io.parity.signer.models.storage.tellRustSeedNames
import io.parity.signer.ui.navigationselectors.OnboardingWasShown
import io.parity.signer.uniffi.*
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import org.json.JSONObject
import java.io.File
import java.io.FileOutputStream

class SignerDataModel : ViewModel() {

	// Internal model values
	private val _onBoardingDone = MutableStateFlow(OnboardingWasShown.InProgress)

	// todo migrate to use dependencies from ServiceLocator rather than expecting them here
	val context: Context get() = ServiceLocator.appContext
	val activity: FragmentActivity get() = ServiceLocator.activityScope!!.activity

	val navigator by lazy { SignerNavigator(this) }

	// Alert
	private val _alertState: MutableStateFlow<AlertState> =
		MutableStateFlow(AlertState.None)

	// Current key details, after rust API will migrate to REST-like should not store this value here.
	internal var lastOpenedKeyDetails: MKeyDetails? = null

	// Transaction
	internal var action = JSONObject()

	val seedStorage: SeedStorage = ServiceLocator.seedStorage

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

	val onBoardingDone: StateFlow<OnboardingWasShown> = _onBoardingDone
	val authenticated: StateFlow<Boolean> = ServiceLocator.authentication.auth

	val alertState: StateFlow<AlertState> = _alertState

	val actionResult: StateFlow<ActionResult> = _actionResult

	val localNavAction: StateFlow<LocalNavAction> = _localNavAction

	// MARK: init boilerplate begin

	/**
	 * Don't forget to call real init after defining context!
	 */
	fun lateInit() {
		// Define local database name
		dbName = context.getDbNameFromContext()

		// Airplane mode detector
		isAirplaneOn()

		val intentFilter = IntentFilter("android.intent.action.AIRPLANE_MODE")

		val receiver: BroadcastReceiver = object : BroadcastReceiver() {
			override fun onReceive(context: Context, intent: Intent) {
				isAirplaneOn()
			}
		}

		context.registerReceiver(receiver, intentFilter)

		// Imitate ios behavior
		val authentication = ServiceLocator.authentication
		authentication.authenticate(activity) {
			seedStorage.init(context)
			totalRefresh()
		}
	}

	/**
	 * Populate database!
	 * This is normal onboarding
	 */
	fun onBoard() {
		wipe()
		copyAsset("")
		historyInitHistoryWithCert()
		totalRefresh()
	}

	/**
	 * Init database with no general certificate
	 */
	private fun jailbreak() {
		wipe()
		copyAsset("")
		historyInitHistoryNoCert()
		totalRefresh()
	}

	/**
	 * Wipes all data
	 */
	@SuppressLint("ApplySharedPref")
	fun wipe() {
		deleteDir(File(dbName))
		seedStorage.wipe()
	}

	/**
	 * Util to copy single Assets file
	 */
	private fun copyFileAsset(path: String) {
		val file = File(dbName, path)
		file.createNewFile()
		val input = context.assets.open("Database$path")
		val output = FileOutputStream(file)
		val buffer = ByteArray(1024)
		var read = input.read(buffer)
		while (read != -1) {
			output.write(buffer, 0, read)
			read = input.read(buffer)
		}
		output.close()
		input.close()
	}

	/**
	 * Util to copy Assets to data dir; only used in onBoard().
	 */
	private fun copyAsset(path: String) {
		val contents = context.assets.list("Database$path")
		if (contents == null || contents.isEmpty()) {
			copyFileAsset(path)
		} else {
			File(dbName, path).mkdirs()
			for (entry in contents) copyAsset("$path/$entry")
		}
	}

	/**
	 * Util to remove directory
	 */
	private fun deleteDir(fileOrDirectory: File) {
		if (fileOrDirectory.isDirectory) {
			val listFiles = fileOrDirectory.listFiles()
			if (!listFiles.isNullOrEmpty()) {
				for (child in listFiles) deleteDir(child)
			}
		}
		fileOrDirectory.delete()
	}

	/**
	 * Checks if airplane mode was off
	 */
	private fun isAirplaneOn() {
		if (Settings.Global.getInt(
				context.contentResolver,
				Settings.Global.AIRPLANE_MODE_ON,
				0
			) == 0
		) {
			if (alertState.value != AlertState.Active) {
				_alertState.value = AlertState.Active
				if (onBoardingDone.value == OnboardingWasShown.Yes) {
					historyDeviceWasOnline()
				}
			}
		} else {
			if (alertState.value == AlertState.Active) {
				_alertState.value = if (onBoardingDone.value == OnboardingWasShown.Yes)
					AlertState.Past else AlertState.None
			}
		}
	}

	// MARK: Init boilerplate end

	// MARK: General utils begin

	/**
	 * This returns the app into starting state; should be called
	 * on all "back"-like events and new screen spawns just in case
	 */
	fun totalRefresh() {
		val checkRefresh = File(dbName).exists()
		if (checkRefresh) _onBoardingDone.value =
			OnboardingWasShown.Yes else _onBoardingDone.value = OnboardingWasShown.No
		if (checkRefresh) {
			val allNames = seedStorage.getSeedNames()
			initNavigation(dbName, allNames.toList())
			getAlertState()
			isAirplaneOn()
			navigator.navigate(Action.START)
		}
	}

	/**
	 * Auth user and wipe the Signer to initial state
	 */
	fun wipeToFactory() {
		val authentication = ServiceLocator.authentication
		authentication.authenticate(activity) {
			wipe()
			totalRefresh()
		}
	}

	/**
	 * Auth user and wipe Signer to state without general verifier certificate
	 */
	fun wipeToJailbreak() {
		val authentication = ServiceLocator.authentication
		authentication.authenticate(activity) {
			jailbreak()
		}
	}

	fun getAppVersion(): String {
		return context.packageManager.getPackageInfo(
			context.packageName,
			0
		).versionName
	}

	private fun getAlertState() {
		_alertState.value = if (historyGetWarnings()) {
			if (alertState.value == AlertState.Active) AlertState.Active else AlertState.Past
		} else {
			AlertState.None
		}
	}

	fun acknowledgeWarning() {
		if (alertState.value == AlertState.Past) {
			historyAcknowledgeWarnings()
			_alertState.value = AlertState.None
		}
	}
}

/**
 * Describes current state of network detection alertness
 */
enum class AlertState {
	None,
	Active,
	Past
}
