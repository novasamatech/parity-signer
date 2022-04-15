package io.parity.signer.models

import android.Manifest
import android.annotation.SuppressLint
import android.content.*
import android.content.pm.PackageManager
import android.os.Build
import android.provider.Settings
import android.util.Log
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import androidx.fragment.app.FragmentActivity
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import androidx.security.crypto.EncryptedSharedPreferences
import androidx.security.crypto.MasterKey
import io.parity.signer.*
import io.parity.signer.components.Authentication
import org.json.JSONObject
import uniffi.signer.*
import java.io.File
import java.io.FileOutputStream

/**
 * This is single object to handle all interactions with backend
 */
class SignerDataModel : ViewModel() {
	private val REQUIRED_PERMISSIONS = arrayOf(Manifest.permission.CAMERA)
	private val REQUEST_CODE_PERMISSIONS = 10

	//Internal model values
	private val _onBoardingDone = MutableLiveData(OnBoardingState.InProgress)

	//TODO: something about this
	// It leaks context objects,
	// but is really quite convenient in composable things
	lateinit var context: Context
	lateinit var activity: FragmentActivity
	private lateinit var masterKey: MasterKey
	private var hasStrongbox: Boolean = false

	//Alert
	private val _alertState = MutableLiveData(ShieldAlert.None)

	//State of the app being unlocked
	private val _authenticated = MutableLiveData(false)

	//Authenticator to call!
	internal var authentication: Authentication =
		Authentication(setAuth = { _authenticated.value = it })

	//Camera stuff
	internal var bucket = arrayOf<String>()
	internal var payload: String = ""
	internal val _total = MutableLiveData<Int?>(null)
	internal val _captured = MutableLiveData<Int?>(null)
	internal val _progress = MutableLiveData(0.0f)

	//Transaction
	internal var action = JSONObject()

	//Internal storage for model data:
	//TODO: hard types for these

	//Seeds
	internal val _seedNames = MutableLiveData(arrayOf<String>())

	//Error
	internal val _lastError = MutableLiveData("")

	//Navigator
	internal val _screen = MutableLiveData(SignerScreen.Log)
	internal val _screenLabel = MutableLiveData("")
	internal val _back = MutableLiveData(false)
	internal val _footerButton = MutableLiveData("")
	internal val _footer = MutableLiveData(false)
	internal val _rightButton = MutableLiveData("None")
	internal val _screenNameType = MutableLiveData("h4")
	internal val _modal = MutableLiveData(SignerModal.Empty)
	internal val _alert = MutableLiveData(SignerAlert.Empty)
	internal var _screenData = MutableLiveData(JSONObject())
	internal var _modalData = MutableLiveData(JSONObject())
	internal var _alertData = MutableLiveData(JSONObject())

	//Data storage locations
	internal var dbName: String = ""
	private val keyStore = "AndroidKeyStore"
	internal lateinit var sharedPreferences: SharedPreferences

	//Observables for model data
	internal val total: LiveData<Int?> = _total
	internal val captured: LiveData<Int?> = _captured
	val progress: LiveData<Float> = _progress

	val seedNames: LiveData<Array<String>> = _seedNames

	val lastError: LiveData<String> = _lastError

	//Observables for screens state

	val onBoardingDone: LiveData<OnBoardingState> = _onBoardingDone
	val authenticated: LiveData<Boolean> = _authenticated

	val alertState: LiveData<ShieldAlert> = _alertState

	val screen: LiveData<SignerScreen> = _screen
	val modal: LiveData<SignerModal> = _modal
	val alert: LiveData<SignerAlert> = _alert
	val screenLabel: LiveData<String> = _screenLabel
	val back: LiveData<Boolean> = _back
	val footer: LiveData<Boolean> = _footer
	val footerButton: LiveData<String> = _footerButton
	val rightButton: LiveData<String> = _rightButton
	val screenNameType: LiveData<String> = _screenNameType
	val screenData: LiveData<JSONObject> = _screenData
	val modalData: LiveData<JSONObject> = _modalData
	val alertData: LiveData<JSONObject> = _alertData

	//MARK: init boilerplate begin

	/**
	 * Init on object creation, context not passed yet! Pass it and call next init
	 */
	init {
		//actually load RustNative code
		System.loadLibrary("signer")
	}

	/**
	 * Don't forget to call real init after defining context!
	 */
	fun lateInit() {
		//Define local database name
		dbName = context.applicationContext.filesDir.toString() + "/Database"
		authentication.context = context
		hasStrongbox = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.P) {
			context.packageManager.hasSystemFeature(PackageManager.FEATURE_STRONGBOX_KEYSTORE)
		} else {
			false
		}
		authentication.strongCredentials = hasStrongbox

		Log.d("strongbox available:", hasStrongbox.toString())

		//Airplane mode detector
		isAirplaneOn()

		val intentFilter = IntentFilter("android.intent.action.AIRPLANE_MODE")

		val receiver: BroadcastReceiver = object : BroadcastReceiver() {
			override fun onReceive(context: Context, intent: Intent) {
				isAirplaneOn()
			}
		}

		context.registerReceiver(receiver, intentFilter)

		//Init crypto for seeds:
		//https://developer.android.com/training/articles/keystore
		masterKey = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.P) {
			MasterKey.Builder(context)
				.setKeyScheme(MasterKey.KeyScheme.AES256_GCM)
				.setRequestStrongBoxBacked(true) //This might cause failures but shouldn't
				.setUserAuthenticationRequired(true)
				.build()
		} else {
			MasterKey.Builder(context)
				.setKeyScheme(MasterKey.KeyScheme.AES256_GCM)
				.setUserAuthenticationRequired(true)
				.build()
		}

		//Imitate ios behavior
		authentication.authenticate(activity) {
			sharedPreferences = EncryptedSharedPreferences(
				context,
				keyStore,
				masterKey,
				EncryptedSharedPreferences.PrefKeyEncryptionScheme.AES256_SIV,
				EncryptedSharedPreferences.PrefValueEncryptionScheme.AES256_GCM
			)
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
		historyInitHistoryWithCert(dbName)
		totalRefresh()
	}

	/**
	 * Init database with no general certificate
	 */
	fun jailbreak() {
		wipe()
		copyAsset("")
		historyInitHistoryNoCert(dbName)
		totalRefresh()
	}

	/**
	 * Wipes all data
	 */
	@SuppressLint("ApplySharedPref")
	fun wipe() {
		deleteDir(File(dbName))
		sharedPreferences.edit().clear().commit() //No, not apply(), do it now!
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
			if (alertState.value != ShieldAlert.Active) {
				_alertState.value = ShieldAlert.Active
				if (onBoardingDone.value == OnBoardingState.Yes) historyDeviceWasOnline(
					dbName
				)
			}
		} else {
			if (alertState.value == ShieldAlert.Active) {
				_alertState.value = if (onBoardingDone.value == OnBoardingState.Yes)
					ShieldAlert.Past else ShieldAlert.None
			}
		}
	}


	private fun allPermissionsGranted() = REQUIRED_PERMISSIONS.all {
		ContextCompat.checkSelfPermission(
			context, it
		) == PackageManager.PERMISSION_GRANTED
	}

	internal fun handleCameraPermissions() {
		if (!allPermissionsGranted()) {
			ActivityCompat.requestPermissions(
				activity,
				REQUIRED_PERMISSIONS,
				REQUEST_CODE_PERMISSIONS
			)
		}
	}

	//MARK: Init boilerplate end

	//MARK: General utils begin

	/**
	 * This returns the app into starting state; should be called
	 * on all "back"-like events and new screen spawns just in case
	 */
	fun totalRefresh() {
		val checkRefresh = File(dbName).exists()
		if (checkRefresh) _onBoardingDone.value =
			OnBoardingState.Yes else _onBoardingDone.value = OnBoardingState.No
		if (checkRefresh) {
			getAlertState()
			refreshSeedNames(init = true)
			pushButton(Action.START)
		}
	}

	/**
	 * Just clears last error;
	 * Run every time user does something
	 */
	fun clearError() {
		_lastError.value = ""
	}

	fun isStrongBoxProtected(): Boolean {
		return masterKey.isStrongBoxBacked
	}

	fun getAppVersion(): String {
		return context.packageManager.getPackageInfo(
			context.packageName,
			0
		).versionName
	}

	private fun getAlertState() {
		_alertState.value = if (historyGetWarnings(dbName)) {
			if (alertState.value == ShieldAlert.Active) ShieldAlert.Active else ShieldAlert.Past
		} else {
			ShieldAlert.None
		}
	}

	fun acknowledgeWarning() {
		if (alertState.value == ShieldAlert.Past) {
			historyAcknowledgeWarnings(dbName)
			_alertState.value = ShieldAlert.None
		}
	}

	//MARK: General utils end

	//MARK: rust native section begin

	//external fun testGetAllTXCards(dbname: String): String
	//external fun testGetAllLogCards(dbname: String): String

	//MARK: rust native section end

}
