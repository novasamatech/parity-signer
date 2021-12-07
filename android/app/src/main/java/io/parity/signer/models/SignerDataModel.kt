package io.parity.signer.models

import android.Manifest
import android.content.Context
import android.content.SharedPreferences
import android.content.pm.PackageManager
import android.util.Log
import androidx.compose.ui.graphics.ImageBitmap
import androidx.fragment.app.FragmentActivity
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import androidx.security.crypto.EncryptedSharedPreferences
import androidx.security.crypto.MasterKey
import io.parity.signer.*
import io.parity.signer.components.Authentication
import org.json.JSONArray
import org.json.JSONObject
import java.io.File
import java.io.FileOutputStream
import android.content.Intent

import android.content.BroadcastReceiver

import android.content.IntentFilter
import android.os.Build
import android.provider.Settings
import androidx.core.content.ContextCompat

/**
 * This is single object to handle all interactions with backend
 */
class SignerDataModel : ViewModel() {
	internal val REQUIRED_PERMISSIONS = arrayOf(Manifest.permission.CAMERA)
	internal val REQUEST_CODE_PERMISSIONS = 10

	//Internal model values
	private val _onBoardingDone = MutableLiveData(OnBoardingState.InProgress)

	//TODO: something about this
	// It leaks context objects,
	// but is really quite convenient in composable things
	lateinit var context: Context
	lateinit var activity: FragmentActivity
	private lateinit var masterKey: MasterKey
	private var hasStrongbox: Boolean = false
	private var _generalCertificate = MutableLiveData(JSONObject())

	//Alert
	private val _alert = MutableLiveData(ShieldAlert.None)

	//Authenticator to call!
	internal var authentication: Authentication = Authentication()

	//Camera stuff
	internal var bucket = arrayOf<String>()
	internal var payload: String = ""
	internal val _total = MutableLiveData<Int?>(null)
	internal val _captured = MutableLiveData<Int?>(null)
	internal val _progress = MutableLiveData(0.0f)

	//Transaction
	internal val _transaction = MutableLiveData(JSONArray())
	internal var action = JSONObject()
	internal val _actionable = MutableLiveData(false)
	var signingAuthor = JSONObject()
	internal var signature = ""

	//Internal storage for model data:
	//TODO: hard types for these

	//Keys
	internal val _identities = MutableLiveData(JSONArray())
	internal val _selectedIdentity = MutableLiveData(JSONObject())

	//Networks
	internal val _networks = MutableLiveData(JSONArray())
	internal val _selectedNetwork = MutableLiveData(JSONObject())

	//Seeds
	internal val _seedNames = MutableLiveData(arrayOf<String>())
	internal val _selectedSeed = MutableLiveData("")

	//TODO: keeping super secret seeds in questionably managed observable must be studied critically
	internal val _backupSeedPhrase = MutableLiveData("")

	//Error
	internal val _lastError = MutableLiveData("")

	//Navigator
	internal val _signerScreen = MutableLiveData(SignerScreen.Log)
	internal val _screenName = MutableLiveData("")
	internal val _backButton = MutableLiveData(false)
	internal val _footerButton = MutableLiveData("")
	internal val _footer = MutableLiveData(false)
	internal val _rightButton = MutableLiveData("None")
	internal val _screenNameType = MutableLiveData("h4")
	internal var screenData = JSONObject()
	internal var modalData = JSONObject()
	internal var alertData = JSONObject()

	//States of important modals
	internal val _signerModal = MutableLiveData(SignerModal.Empty)
	internal val _transactionState = MutableLiveData(TransactionState.None)

	internal val _signerAlert = MutableLiveData(SignerAlert.Empty)

	//Data storage locations
	internal var dbName: String = ""
	private val keyStore = "AndroidKeyStore"
	internal lateinit var sharedPreferences: SharedPreferences

	//Observables for model data
	internal val total: LiveData<Int?> = _total
	internal val captured: LiveData<Int?> = _captured
	val progress: LiveData<Float> = _progress

	val transaction: LiveData<JSONArray> = _transaction
	val actionable: LiveData<Boolean> = _actionable

	val identities: LiveData<JSONArray> = _identities
	val selectedIdentity: LiveData<JSONObject> = _selectedIdentity

	val networks: LiveData<JSONArray> = _networks
	val selectedNetwork: LiveData<JSONObject> = _selectedNetwork

	val seedNames: LiveData<Array<String>> = _seedNames
	val selectedSeed: LiveData<String> = _selectedSeed
	val backupSeedPhrase: LiveData<String> = _backupSeedPhrase

	var generalCertificate: LiveData<JSONObject> = _generalCertificate

	val lastError: LiveData<String> = _lastError

	//Observables for screens state

	val onBoardingDone: LiveData<OnBoardingState> = _onBoardingDone
	val signerScreen: LiveData<SignerScreen> = _signerScreen
	val signerModal: LiveData<SignerModal> = _signerModal
	val signerAlert: LiveData<SignerAlert> = _signerAlert
	val transactionState: LiveData<TransactionState> = _transactionState
	val alert: LiveData<ShieldAlert> = _alert
	val screenName: LiveData<String> = _screenName
	val backButton: LiveData<Boolean> = _backButton
	val footer: LiveData<Boolean> = _footer
	val footerButton: LiveData<String> = _footerButton
	val rightButton: LiveData<String> = _rightButton
	val screenNameType: LiveData<String> = _screenNameType

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

		Log.d("strongbox available:", hasStrongbox.toString())

		//Airplane mode detector
		isAirplaneOn()

		val intentFilter = IntentFilter("android.intent.action.AIRPLANE_MODE")

		val receiver: BroadcastReceiver = object : BroadcastReceiver() {
			override fun onReceive(context: Context, intent: Intent) {
				Log.d("AirplaneMode", "Service state changed")
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
			if (alert.value != ShieldAlert.Active) {
				_alert.value = ShieldAlert.Active
				historyDeviceWasOnline(dbName)
			}
		} else {
			if (alert.value == ShieldAlert.Active) {
				_alert.value = ShieldAlert.Past
			}
		}
	}

	/**
	 * Gets general verifier value from db
	 */
	internal fun getGeneralVerifier() {
		try {
			_generalCertificate.value = JSONObject(dbGetGeneralVerifier(dbName))
		} catch (e: java.lang.Exception) {
			Log.e("General verifier fetch error", e.toString())
		}
	}

	internal fun allPermissionsGranted() = REQUIRED_PERMISSIONS.all {
		ContextCompat.checkSelfPermission(
			context, it
		) == PackageManager.PERMISSION_GRANTED
	}

	//MARK: Init boilerplate end

	//MARK: General utils begin

	fun refreshGUI() {
		_backupSeedPhrase.value = ""
		clearError()
		_transactionState.value = TransactionState.None
		_signerScreen.value = if (seedNames.value?.isEmpty() as Boolean)
			 SignerScreen.NewSeed else SignerScreen.Log
	}

	/**
	 * This returns the app into starting state; should be called
	 * on all "back"-like events and new screen spawns just in case
	 */
	fun totalRefresh() {
		_backupSeedPhrase.value = ""
		val checkRefresh = File(dbName).exists()
		if (checkRefresh) _onBoardingDone.value =
			OnBoardingState.Yes else _onBoardingDone.value = OnBoardingState.No
		if (checkRefresh) {
			initNavigation(dbName, seedNames.value?.joinToString(",")?:"")
			pushButton(ButtonID.Start)
			refreshNetworks()
			_selectedNetwork.value = networks.value?.optJSONObject(0)
				?: JSONObject()
			refreshSeedNames()
			fetchKeys()
			refreshGUI()
			getGeneralVerifier()
			clearTransaction()
			if (signerScreen.value == null) pushButton(ButtonID.NavbarLog)
		}
	}

	/**
	 * Just clears last error;
	 * Run every time user does something
	 */
	fun clearError() {
		_lastError.value = ""
	}

	/**
	 * Generate identicon from some form of address:
	 * this is only a wrapper and converter to png.
	 * This should not fail!
	 */
	fun getIdenticon(key: String, size: Int): ImageBitmap {
		return try {
			substrateBase58Identicon(key, size).intoImageBitmap()
		} catch (e: java.lang.Exception) {
			Log.d("Identicon did not render", e.toString())
			ImageBitmap(size, size)
		}
		//TODO
	}

	fun getHexIdenticon(value: String, size: Int): ImageBitmap {
		return try {
			substrateIdenticon(value, size).intoImageBitmap()
		} catch (e: java.lang.Exception) {
			Log.d("Identicon did not render", e.toString())
			ImageBitmap(size, size)
		}
		//TODO
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

	//MARK: General utils end

	//MARK: rust native section begin

	external fun backendAction(
		action: String,
		details: String
	): String

	external fun initNavigation(
		dbname: String,
		seedNames: String
	);

	external fun substrateExportPubkey(
		address: String,
		network: String,
		dbname: String
	): String

	external fun qrparserGetPacketsTotal(data: String, cleaned: Boolean): Int
	external fun qrparserTryDecodeQrSequence(
		data: String,
		cleaned: Boolean
	): String

	external fun substrateParseTransaction(
		transaction: String,
		dbname: String
	): String

	external fun substrateHandleStub(checksum: String, dbname: String)

	external fun substrateHandleSign(
		checksum: String,
		seedPhrase: String,
		password: String,
		userComment: String,
		dbname: String
	): String

	external fun substrateBase58Identicon(base58: String, size: Int): String
	external fun substrateIdenticon(key: String, size: Int): String
	external fun dbGetNetwork(genesisHash: String, dbname: String): String
	external fun dbGetAllNetworksForNetworkSelector(dbname: String): String
	external fun dbGetRelevantIdentities(
		seedName: String,
		genesisHash: String,
		dbname: String
	): String

	external fun dbGetAllIdentities(dbname: String): String
	external fun substrateTryCreateSeed(
		seedName: String,
		seedPhrase: String,
		seedLength: Int,
		dbname: String
	): String

	external fun substrateSuggestNPlusOne(
		path: String,
		seedName: String,
		networkIdString: String,
		dbname: String
	): String

	external fun substrateCheckPath(path: String): Boolean
	external fun substrateTryCreateIdentity(
		seedName: String,
		seedPhrase: String,
		crypto: String,
		path: String,
		network: String,
		hasPassword: Boolean,
		dbname: String
	)

	external fun substrateDeleteIdentity(
		pubKey: String,
		network: String,
		dbname: String
	)

	external fun substrateGetNetworkSpecs(network: String, dbname: String): String
	external fun substrateRemoveNetwork(network: String, dbname: String)
	external fun substrateRemoveMetadata(
		networkName: String,
		networkVersion: Int,
		dbname: String
	)

	external fun substrateRemoveSeed(seedName: String, dbname: String)
	external fun historyClearHistory(dbname: String)
	external fun historyInitHistoryWithCert(dbname: String)
	external fun historyInitHistoryNoCert(dbname: String)
	external fun historyDeviceWasOnline(dbname: String)
	external fun historyGetWarnings(dbname: String): Boolean
	external fun historyAcknowledgeWarnings(dbname: String)
	external fun historyEntryUser(entry: String, dbname: String)
	external fun historyEntrySystem(entry: String, dbname: String)
	external fun historyHistorySeedNameWasShown(seedName: String, dbname: String)
	external fun dbGetGeneralVerifier(dbname: String): String
	external fun signerSignTypes(
		publicKey: String,
		encryption: String,
		seedPhrase: String,
		password: String,
		dbname: String
	): String

	external fun signerSignMetadata(
		network: String,
		version: Int,
		publicKey: String,
		encryption: String,
		seedPhrase: String,
		password: String,
		dbname: String
	): String

	external fun signerSignSpecs(
		network: String,
		publicKey: String,
		encryption: String,
		seedPhrase: String,
		password: String,
		dbname: String
	): String

	external fun testGetAllTXCards(dbname: String): String
	external fun testGetAllLogCards(dbname: String): String

	//MARK: rust native section end

}
