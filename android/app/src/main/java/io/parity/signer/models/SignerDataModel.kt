package io.parity.signer.models

import android.Manifest
import android.annotation.SuppressLint
import android.content.Context
import android.content.SharedPreferences
import android.content.pm.PackageManager
import android.util.Log
import androidx.camera.core.ImageProxy
import androidx.compose.ui.graphics.ImageBitmap
import androidx.fragment.app.FragmentActivity
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import androidx.security.crypto.EncryptedSharedPreferences
import androidx.security.crypto.MasterKey
import com.google.mlkit.vision.barcode.BarcodeScanner
import com.google.mlkit.vision.common.InputImage
import io.parity.signer.*
import io.parity.signer.components.Authentication
import org.json.JSONArray
import org.json.JSONObject
import java.io.File
import java.io.FileOutputStream
import android.content.Intent

import android.content.BroadcastReceiver

import android.content.IntentFilter
import android.provider.Settings
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat


//TODO: chop this monster in chunks

/**
 * This is single object to handle all interactions with backend
 */
class SignerDataModel : ViewModel() {
	private val REQUIRED_PERMISSIONS = arrayOf(Manifest.permission.CAMERA)
	private val REQUEST_CODE_PERMISSIONS = 10

	//Internal model values
	private val _onBoardingDone = MutableLiveData(OnBoardingState.InProgress)
	lateinit var context: Context
	lateinit var activity: FragmentActivity
	lateinit var masterKey: MasterKey
	private var hasStrongbox: Boolean = false
	private var _generalCertificate = MutableLiveData(JSONObject())

	//Alert
	private val _alert = MutableLiveData(SignerAlert.None)

	//Authenticator to call!
	var authentication: Authentication = Authentication()

	//Camera stuff
	private var bucket = arrayOf<String>()
	private var payload: String = ""
	private val _total = MutableLiveData<Int?>(null)
	private val _captured = MutableLiveData<Int?>(null)
	private val _progress = MutableLiveData<Float>(0.0f)

	//Transaction
	private val _transaction = MutableLiveData(JSONArray())
	private var action = JSONObject()
	private val _actionable = MutableLiveData(false)
	private var signingAuthor = JSONObject()
	private var signature = ""

	//Internal storage for model data:
	//TODO: hard types for these

	//Keys
	private val _identities = MutableLiveData(JSONArray())
	private val _selectedIdentity = MutableLiveData(JSONObject())

	//Networks
	private val _networks = MutableLiveData(JSONArray())
	private val _selectedNetwork = MutableLiveData(JSONObject())

	//Seeds
	private val _seedNames = MutableLiveData(arrayOf<String>())
	private val _selectedSeed = MutableLiveData("")

	//TODO: keeping super secret seeds in questionably managed observable must be studied critically
	private val _backupSeedPhrase = MutableLiveData("")

	//History
	private val _history = MutableLiveData(JSONArray())

	//Error
	private val _lastError = MutableLiveData("")

	//Navigator
	private val _signerScreen = MutableLiveData(SignerScreen.Log)

	//States of important modals
	private val _keyManagerModal = MutableLiveData(KeyManagerModal.None)
	private val _settingsModal = MutableLiveData(SettingsModal.None)
	private val _transactionState = MutableLiveData(TransactionState.None)

	//Data storage locations
	private var dbName: String = ""
	private val keyStore = "AndroidKeyStore"
	private lateinit var sharedPreferences: SharedPreferences

	//Observables for model data
	val total: LiveData<Int?> = _total
	val captured: LiveData<Int?> = _captured
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

	val history: LiveData<JSONArray> = _history

	var generalCertificate: LiveData<JSONObject> = _generalCertificate

	val lastError: LiveData<String> = _lastError

	//Observables for screens state

	val onBoardingDone: LiveData<OnBoardingState> = _onBoardingDone
	val signerScreen: LiveData<SignerScreen> = _signerScreen
	val keyManagerModal: LiveData<KeyManagerModal> = _keyManagerModal
	val settingsModal: LiveData<SettingsModal> = _settingsModal
	val transactionState: LiveData<TransactionState> = _transactionState
	val alert: LiveData<SignerAlert> = _alert

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
		hasStrongbox =
			context.packageManager.hasSystemFeature(PackageManager.FEATURE_STRONGBOX_KEYSTORE)

		Log.d("strongbox available", hasStrongbox.toString())

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
		masterKey = MasterKey.Builder(context)
			.setKeyScheme(MasterKey.KeyScheme.AES256_GCM)
			.setRequestStrongBoxBacked(hasStrongbox) // this must be default, but...
			.setUserAuthenticationRequired(true)
			.build()

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
		sharedPreferences.edit().clear().commit()
	}

	/**
	 * Util to copy single Assets file
	 */
	private fun copyFileAsset(path: String) {
		var file = File(dbName, path)
		file.createNewFile()
		var input = context.assets.open("Database$path")
		var output = FileOutputStream(file)
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
		if (fileOrDirectory.isDirectory()) for (child in fileOrDirectory.listFiles()) deleteDir(
			child
		)
		fileOrDirectory.delete()
	}

	/**
	 * Checks if airplane mode was off
	 */
	private fun isAirplaneOn() {
		if (Settings.Global.getInt(context.contentResolver, Settings.Global.AIRPLANE_MODE_ON, 0) == 0) {
			if (alert.value != SignerAlert.Active) {
				_alert.value = SignerAlert.Active
				historyDeviceWasOnline(dbName)
			}
		} else {
			if (alert.value == SignerAlert.Active) {
				_alert.value = SignerAlert.Past
			}
		}
	}

	/**
	 * Gets general verifier value from db
	 */
	private fun getGeneralVerifier() {
		try {
			_generalCertificate.value = JSONObject(dbGetGeneralVerifier(dbName))
		} catch (e: java.lang.Exception) {
			Log.e("General verifier fetch error", e.toString())
		}
	}

	private fun allPermissionsGranted() = REQUIRED_PERMISSIONS.all {
		ContextCompat.checkSelfPermission(
			context, it
		) == PackageManager.PERMISSION_GRANTED
	}

	//MARK: Init boilerplate end

	//MARK: General utils begin

	/**
	 * This returns the app into starting state; should be called
	 * on all "back"-like events and new screen spawns just in case
	 */
	fun totalRefresh() {
		_backupSeedPhrase.value = ""
		clearError()
		val checkRefresh = File(dbName).exists()
		if (checkRefresh) _onBoardingDone.value =
			OnBoardingState.Yes else _onBoardingDone.value = OnBoardingState.No
		if (checkRefresh) {
			refreshNetworks()
			//TODO: support state with all networks deleted (low priority)
			if (true) {
				_selectedNetwork.value = networks.value!!.get(0) as JSONObject
			}
			refreshSeedNames()
			_transactionState.value = TransactionState.None
			_settingsModal.value = SettingsModal.None
			if (seedNames.value?.isEmpty() as Boolean) _keyManagerModal.value =
				KeyManagerModal.NewSeed else _keyManagerModal.value =
				KeyManagerModal.None
			fetchKeys()
			refreshHistory()
		}
		clearTransaction()
	}

	/**
	 * Get history from db; should bhe run on log screen appearance
	 */
	fun refreshHistory() {
		try {
			_history.value = sortHistory(JSONArray(historyPrintHistory(dbName)))
			_alert.value = if (historyGetWarnings(dbName)) {SignerAlert.Past} else {SignerAlert.None}
		} catch (e: java.lang.Exception) {
			Log.e("History refresh error!", e.toString())
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

	//MARK: General utils end

	//MARK: Transaction utils begin

	/**
	 * Send scanned QR to backend and rearrange cards nicely
	 * We should probably simplify this once UI development is done
	 */
	fun parseTransaction() {
		_transactionState.value = TransactionState.Parsing
		try {
			val transactionString = substrateParseTransaction(payload, dbName)
			Log.d("transaction string", transactionString)
			val transactionObject = JSONObject(transactionString)
			//TODO: something here
			val author = (transactionObject.optJSONArray("author") ?: JSONArray())
			val warnings = transactionObject.optJSONArray("warning") ?: JSONArray()
			val error = (transactionObject.optJSONArray("error") ?: JSONArray())
			val typesInfo =
				transactionObject.optJSONArray("types_info") ?: JSONArray()
			val method = (transactionObject.optJSONArray("method") ?: JSONArray())
			val extensions =
				(transactionObject.optJSONArray("extensions") ?: JSONArray())
			val newSpecs = (transactionObject.optJSONArray("new_specs") ?: JSONArray())
			val verifier = (transactionObject.optJSONArray("verifier") ?: JSONArray())
			action = transactionObject.optJSONObject("action") ?: JSONObject()
			_actionable.value = !action.isNull("type")
			if (action.optString("type") == "sign") {
				signingAuthor = author.getJSONObject(0)
			}
			Log.d("action", action.toString())
			_transaction.value =
				sortCards(
					concatJSONArray(
						author,
						warnings,
						error,
						typesInfo,
						method,
						extensions,
						newSpecs,
						verifier
					)
				)
			_transactionState.value = TransactionState.Preview
		} catch (e: java.lang.Exception) {
			Log.e("Transaction parsing failed", e.toString())
			_transactionState.value = TransactionState.None
		}
	}

	fun acceptTransaction() {
		if (action.getString("type") == "sign") {
			Log.d("authorcard", signingAuthor.toString())
			if (signingAuthor.getJSONObject("payload").getBoolean("has_password")) {
				_transactionState.value = TransactionState.Password
			} else {
				signTransaction("")
			}
		} else {
			performTransaction()
			clearTransaction()
		}
	}

	fun signTransaction(password: String) {
		authentication.authenticate(activity) {
			signature = substrateHandleSign(
				action.getString(
					"payload"), sharedPreferences.getString(
					signingAuthor.getJSONObject("payload").getString("seed"), ""
				) ?: "", password, "", dbName
			)
			_transactionState.value = TransactionState.Signed
		}
	}

	fun getSignedQR(): ImageBitmap {
		return signature.intoImageBitmap()
	}

	private fun performTransaction() {
		try {
			substrateHandleStub(
				action.getString("payload"),
				dbName
			)
		} catch (e: java.lang.Exception) {
			Log.e("transaction failed", e.toString())
			_lastError.value = e.toString()
		}
	}

	/**
	 * Clear all transaction progress side effects
	 */
	fun clearTransaction() {
		signature = ""
		action = JSONObject()
		signingAuthor = JSONObject()
		_transaction.value = JSONArray()
		_actionable.value = false
		_transactionState.value = TransactionState.None
		resetScan()
	}

	//MARK: Transaction utils end

	//MARK: Camera tools begin

	/**
	 * Barcode detecting function.
	 * This uses experimental features
	 */
	@SuppressLint("UnsafeOptInUsageError")
	fun processFrame(barcodeScanner: BarcodeScanner, imageProxy: ImageProxy) {
		val inputImage = InputImage.fromMediaImage(
			imageProxy.image,
			imageProxy.imageInfo.rotationDegrees
		)

		barcodeScanner.process(inputImage)
			.addOnSuccessListener { barcodes ->
				barcodes.forEach {
					val payloadString = it?.rawBytes?.encodeHex()
					Log.d("QR", payloadString ?: "empty")
					if (!(bucket.contains(payloadString) || payloadString.isNullOrEmpty())) {
						if (total.value == null) {
							try {
								val proposeTotal = qrparserGetPacketsTotal(payloadString, true)
								Log.d("estimate total", proposeTotal.toString())
								if (proposeTotal == 1) {
									try {
										payload = qrparserTryDecodeQrSequence(
											"[\"" + payloadString + "\"]",
											true
										)
										resetScan()
										parseTransaction()
										Log.d("payload", payload)
									} catch (e: java.lang.Exception) {
										Log.e("Single frame decode failed", e.toString())
									}
								} else {
									bucket += payloadString
									_total.value = proposeTotal
								}
							} catch (e: java.lang.Exception) {
								Log.e("QR sequence length estimation", e.toString())
							}
						} else {
							bucket += payloadString
							if (bucket.size >= total.value ?: 0) {
								try {
									payload = qrparserTryDecodeQrSequence(
										"[\"" + bucket.joinToString(separator = "\",\"") + "\"]",
										true
									)
									Log.d("multiframe payload", payload)
									if (!payload.isEmpty()) {
										resetScan()
										parseTransaction()
									}
								} catch (e: java.lang.Exception) {
									Log.e("failed to parse sequence", e.toString())
								}
							}
							_captured.value = bucket.size
							_progress.value = ((captured.value ?: 0).toFloat() / ((total.value
								?: 1).toFloat()))
							Log.d("captured", captured.value.toString())
						}
					}
				}
			}
			.addOnFailureListener {
				Log.e("Scan failed", it.message.toString())
			}
			.addOnCompleteListener {
				imageProxy.close()
			}
	}

	/**
	 * Clears camera progress
	 */
	fun resetScan() {
		bucket = arrayOf<String>()
		_captured.value = null
		_total.value = null
		_progress.value = 0.0f
	}

	//MARK: Camera tools end

	//MARK: Seed management begin

	/**
	 * Refresh seed names list
	 * should be called within authentication envelope
	 * authentication.authenticate(activity) {refreshSeedNames()}
	 * which is somewhat asynchronous
	 */
	fun refreshSeedNames() {
		clearError()
		_seedNames.value = sharedPreferences.all.keys.toTypedArray()
	}

	/**
	 * Add seed, encrypt it, and create default accounts
	 */
	fun addSeed(seedName: String, seedPhrase: String) {

		//Check if seed name already exists
		if (seedNames.value?.contains(seedName) as Boolean) {
			_lastError.value = "Seed with this name already exists!"
		}

		//Run standard login prompt!
		authentication.authenticate(activity) {
			try {
				//Create relevant keys - should make sure this works before saving key
				var finalSeedPhrase =
					substrateTryCreateSeed(seedName, seedPhrase, 24, dbName)

				//Encrypt and save seed
				with(sharedPreferences.edit()) {
					putString(seedName, finalSeedPhrase)
					apply()
				}

				//Refresh model
				refreshSeedNames()
				selectSeed(seedName)
				_backupSeedPhrase.value = finalSeedPhrase
				_keyManagerModal.value = KeyManagerModal.SeedBackup
			} catch (e: java.lang.Exception) {
				_lastError.value = e.toString()
				Log.e("Seed creation error", e.toString())
			}
		}
	}

	/**
	 * Seed selector; does not check if seedname is valid
	 * TODO: check that all related operations are done
	 */
	fun selectSeed(seedName: String) {
		_selectedSeed.value = seedName
		fetchKeys()
	}

	/**
	 * Fetch seed from strongbox; must be in unlocked scope
	 */
	fun getSeed(): String {
		return sharedPreferences.getString(selectedSeed.value, "") ?: ""
	}

	/**
	 * Selects seed key, if available
	 */
	fun getRootIdentity(seedName: String): JSONObject {
		for (i in 0 until identities.value!!.length()) {
			val identity = identities.value!!.getJSONObject(i)
			if (identity.getString("seed_name") == seedName && identity.getString("path") == "" && identity.getString(
					"has_password"
				) == "false"
			) {
				return identity
			}
		}
		return JSONObject()
	}

	//MARK: Seed management end

	//MARK: Network management begin

	/**
	 * Get network list updated; call after any networks-altering operation
	 * and on init and on refresh just in case
	 */
	fun refreshNetworks() {
		try {
			val networkJSON = dbGetAllNetworksForNetworkSelector(dbName)
			_networks.value = JSONArray(networkJSON)
			fetchKeys()
		} catch (e: java.lang.Exception) {
			Log.e("Refresh network error", e.toString())
		}
	}


	fun selectNetwork(network: JSONObject) {
		_selectedNetwork.value = network
		fetchKeys()
	}

	//MARK: Network management end

	//MARK: Key management begin

	/**
	 * Refresh keys relevant for other parameters
	 */
	fun fetchKeys() {
		try {
			Log.d("selectedNetwork", selectedNetwork.value.toString())
			Log.d("Selected seed", selectedSeed.value.toString())
			_identities.value = JSONArray(
				dbGetRelevantIdentities(
					selectedSeed.value ?: "",
					selectedNetwork.value?.get("key").toString(),
					dbName
				)
			)
		} catch (e: java.lang.Exception) {
			Log.e("fetch keys error", e.toString())
		}
	}

	/**
	 * Just set key for filtering
	 */
	fun selectKey(key: JSONObject) {
		_selectedIdentity.value = key
	}

	/**
	 * Add key to database; uses phone crypto to fetch seeds!
	 */
	fun addKey(path: String, password: String) {
		if (selectedSeed.value?.isEmpty() as Boolean) selectSeed(
			selectedIdentity.value?.get(
				"seed_name"
			).toString()
		)
		var fullPath = path
		val hasPassword = !password.isEmpty()
		if (hasPassword) fullPath += "///" + password
		try {
			if (substrateCheckPath(path) != hasPassword) {
				_lastError.value =
					"The sequence /// is not allowed in path; use password field to include password (omitting ///)"
				Log.e("Add key preparation error", "password in path field")
				return
			}
		} catch (e: java.lang.Exception) {
			_lastError.value = e.toString()
			Log.e("Add key path check error", e.toString())
		}
		authentication.authenticate(activity) {
			try {
				substrateTryCreateIdentity(
					selectedSeed.value!!,
					getSeed(),
					"sr25519",
					path,
					selectedNetwork.value?.get("key").toString(),
					hasPassword,
					dbName
				)
				fetchKeys()
				clearKeyManagerScreen()
			} catch (e: java.lang.Exception) {
				Log.e("Add key error", e.toString())
				_lastError.value = e.toString()
			}
		}
	}

	/**
	 * delete selected key for selected network
	 */
	fun deleteKey() {
		try {
			substrateDeleteIdentity(
				selectedIdentity.value?.get("public_key").toString(),
				selectedNetwork.value?.get("key").toString(),
				dbName
			)
			fetchKeys()
			clearKeyManagerScreen()
		} catch (e: java.lang.Exception) {
			Log.e("key deletion error", e.toString())
		}
	}

	fun proposeDerivePath(): String {
		return if (selectedIdentity.value?.isNull("path") as Boolean)
			"//"
		else
			selectedIdentity.value?.get("path").toString()
	}

	fun proposeIncrement(): String {
		if (selectedIdentity.value?.isNull("path") as Boolean)
			return ""
		else {
			return try {
				substrateSuggestNPlusOne(
					selectedIdentity.value?.get("path").toString(),
					selectedSeed.value.toString(),
					selectedNetwork.value?.get("key").toString(),
					dbName
				)
			} catch (e: java.lang.Exception) {
				_lastError.value = e.toString()
				Log.e("Increment error", e.toString())
				""
			}
		}
	}

	fun exportPublicKey(): ImageBitmap {
		return try {
			substrateExportPubkey(
				selectedIdentity.value?.get("public_key").toString(),
				selectedNetwork.value?.get("key").toString(),
				dbName
			).intoImageBitmap()
		} catch (e: java.lang.Exception) {
			Log.d("QR export error", e.toString())
			_lastError.value = e.toString()
			ImageBitmap(1, 1)
		}
	}

	//MARK: Key management end

	//MARK: Navigation begin

	/**
	 * Bottom navigation action
	 */
	fun navigate(screen: SignerScreen) {
		_signerScreen.value = screen
		if (screen == SignerScreen.Scan) {
			//TODO: testing to make sure this goes smoothly
			if (!allPermissionsGranted()) {
				ActivityCompat.requestPermissions(
					activity,
					REQUIRED_PERMISSIONS,
					REQUEST_CODE_PERMISSIONS
				)
			}
		}
		if (screen == SignerScreen.Keys) {
			selectSeedEngage()
		}
		if (screen == SignerScreen.Log) {
			engageHistoryScreen()
		}
	}

	/**
	 * Handle back button
	 */
	fun goBack() {
		when (signerScreen.value) {
			SignerScreen.Log -> {
				totalRefresh()
			}
			SignerScreen.Scan -> {
				clearTransaction()
			}
			SignerScreen.Keys -> {
				when (keyManagerModal.value) {
					KeyManagerModal.None -> {
						selectSeedEngage()
					}
					KeyManagerModal.NewSeed -> {
						selectSeedEngage()
					}
					KeyManagerModal.NewKey -> {
						clearKeyManagerScreen()
					}
					KeyManagerModal.ShowKey -> {
						clearKeyManagerScreen()
					}
					KeyManagerModal.SeedBackup -> {
						selectSeedEngage()
					}
					KeyManagerModal.KeyDeleteConfirm -> {
						clearKeyManagerScreen()
					}
					KeyManagerModal.SeedSelector -> {
						selectSeedEngage()
					}
					KeyManagerModal.NetworkManager -> {
						clearKeyManagerScreen()
					}
					KeyManagerModal.NetworkDetails -> {
						clearKeyManagerScreen()
					}
				}
			}
			SignerScreen.Settings -> {
				clearHistoryScreen()
			}
		}
	}

	fun isBottom(): Boolean {
		return (settingsModal.value == SettingsModal.None && keyManagerModal.value == KeyManagerModal.SeedSelector && transactionState.value == TransactionState.None)
	}

	fun getScreenName(): String {
		Log.d("getscreenname", "called")
		return when (signerScreen.value) {
			SignerScreen.Scan -> ""
			SignerScreen.Keys -> when (keyManagerModal.value) {
				KeyManagerModal.None -> ""
				KeyManagerModal.NewSeed -> ""
				KeyManagerModal.NewKey -> "New Derived Key"
				KeyManagerModal.ShowKey -> if (selectedIdentity.value == getRootIdentity(
						selectedSeed.value ?: ""
					)
				) {
					"Seed key"
				} else {
					"Derived Key"
				}
				KeyManagerModal.SeedBackup -> "Backup Seed"
				KeyManagerModal.KeyDeleteConfirm -> ""
				KeyManagerModal.SeedSelector -> "Select Seed"
				KeyManagerModal.NetworkManager -> ""
				KeyManagerModal.NetworkDetails -> ""
				null -> "error"
			}
			SignerScreen.Settings -> ""
			SignerScreen.Log -> ""
			null -> "error"
		}
	}
	//MARK: Navigation end

	//MARK: Modals control begin

	//KeyManager

	/**
	 * This happens when backup seed acknowledge button is pressed in seed creation screen.
	 * TODO: This might misfire
	 */
	fun acknowledgeBackup() {
		_backupSeedPhrase.value = ""
		clearKeyManagerScreen()
	}

	/**
	 * Use this to bring up seed selection screen in key manager
	 */
	fun selectSeedEngage() {
		selectSeed("")
		_keyManagerModal.value = KeyManagerModal.SeedSelector
	}

	/**
	 * Activate new seed screen on KeyManager screen
	 */
	fun newSeedScreenEngage() {
		_keyManagerModal.value = KeyManagerModal.NewSeed
	}

	/**
	 * Derive new key
	 */
	fun newKeyScreenEngage() {
		_keyManagerModal.value = KeyManagerModal.NewKey
	}

	/**
	 * Show public key QR screen
	 */
	fun exportPublicKeyEngage() {
		_keyManagerModal.value = KeyManagerModal.ShowKey
	}

	/**
	 * Remove key manager modals
	 */
	fun clearKeyManagerScreen() {
		_keyManagerModal.value = if (selectedSeed.value == "") {
			KeyManagerModal.SeedSelector
		} else {
			KeyManagerModal.None
		}
	}

	/**
	 * Key deletion confirmation
	 */
	fun deleteKeyConfirmation() {
		_keyManagerModal.value = KeyManagerModal.KeyDeleteConfirm
	}

	//Settings

	fun engageHistoryScreen() {
		refreshHistory()
		getGeneralVerifier()
		_signerScreen.value = SignerScreen.Log
	}

	fun clearHistoryScreen() {
		_settingsModal.value = SettingsModal.None
	}

	//MARK: Modals control end

	//MARK: rust native section begin

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
	external fun historyPrintHistory(dbname: String): String
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

/*
		.setKeyGenParameterSpec(
			KeyGenParameterSpec
				.Builder(
					MasterKey.DEFAULT_MASTER_KEY_ALIAS,
					KeyProperties.PURPOSE_ENCRYPT or KeyProperties.PURPOSE_DECRYPT
				)
				.setBlockModes(KeyProperties.BLOCK_MODE_GCM)
				.setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_NONE)
				.setKeySize(MasterKey.DEFAULT_AES_GCM_MASTER_KEY_SIZE)
				//.setUserAuthenticationParameters(1, KeyProperties.AUTH_DEVICE_CREDENTIAL)
				//.setUserAuthenticationRequired(true)
				.setIsStrongBoxBacked(hasStrongbox)
				.build()
		)*/
