package io.parity.signer.models

import android.app.Activity
import android.app.PendingIntent.getActivity
import android.content.Context
import android.content.SharedPreferences
import android.content.pm.PackageManager
import android.security.keystore.KeyGenParameterSpec
import android.security.keystore.KeyProperties
import android.util.Log
import androidx.compose.ui.platform.LocalContext
import androidx.fragment.app.FragmentActivity
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import androidx.security.crypto.EncryptedSharedPreferences
import androidx.security.crypto.MasterKey
import io.parity.signer.MainActivity
import io.parity.signer.components.Authentication
import org.json.JSONArray
import org.json.JSONObject
import java.io.File
import java.io.FileOutputStream
import java.io.IOException
import java.security.KeyStore
import javax.crypto.KeyGenerator

/**
 * This is single object to handle all interactions with backend,
 * except for some logging features and transaction handling
 */
class SignerDataModel: ViewModel() {
	//Internal model values
	private val _onBoardingDone = MutableLiveData(false)
	private val _developmentTest = MutableLiveData("")

	//TODO: hard types for these
	private val _networks = MutableLiveData(JSONArray())
	private val _selectedNetwork = MutableLiveData(JSONObject())

	private val _seedNames = MutableLiveData(arrayOf<String>())

	private val _lastError = MutableLiveData("")

	//Data storage locations
	private var dbName: String = ""
	private val keyStore = "AndroidKeyStore"
	private val keyStoreName = "SignerSeedStorage"
	private lateinit var sharedPreferences: SharedPreferences

	//Observables
	val onBoardingDone: LiveData<Boolean> = _onBoardingDone
	val developmentTest: LiveData<String> = _developmentTest
	val networks: LiveData<JSONArray> = _networks
	val selectedNetwork: LiveData<JSONObject> = _selectedNetwork

	val seedNames: LiveData<Array<String>> = _seedNames

	val lastError: LiveData<String> = _lastError

	lateinit var context: Context
	lateinit var activity: FragmentActivity
	lateinit var masterKey: MasterKey
	private var hasStrongbox: Boolean = false
	var authentication: Authentication = Authentication()

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
		hasStrongbox = context.packageManager.hasSystemFeature(PackageManager.FEATURE_STRONGBOX_KEYSTORE)

		Log.d("strongbox available", hasStrongbox.toString())

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
	 */
	fun onBoard() {
		copyAsset("")
		totalRefresh()
	}

	/**
	 * TODO: wipe all data!
	 */
	fun wipe() {
		File(dbName).delete()
	}

	fun initEncryptedSharedPreferences() {
		//2. Generate storage - incompatible with internals of biometry lib, a pity
		sharedPreferences = EncryptedSharedPreferences(
			context,
			keyStore,
			masterKey,
			EncryptedSharedPreferences.PrefKeyEncryptionScheme.AES256_SIV,
			EncryptedSharedPreferences.PrefValueEncryptionScheme.AES256_GCM
		)
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
		if (contents == null || contents.size == 0) {
			copyFileAsset(path)
		} else {
			File(dbName, path).mkdirs()
			for (entry in contents) copyAsset("$path/$entry")
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
		_onBoardingDone.value = checkRefresh
		if (checkRefresh) {

			refreshNetworks()
			//TODO: support state with all networks deleted (low priority)
			if (true) {
				_selectedNetwork.value = networks.value!!.get(0) as JSONObject
			}
			refreshSeedNames()
		}
	}

	//TODO: development function; should be removed on release
	fun callNative(input: String): String {
		var test: String
		try {
			test = substrateDevelopmentTest(input)
		} catch (e: Exception) {
			test = e.toString()
		}
		return test
	}

	//MARK: General utils end

	//MARK: Seed management begin

	/**
	 * Refresh seed names list
	 */
	fun refreshSeedNames() {
		_seedNames.value = sharedPreferences.all.keys.toTypedArray()
	}

	/**
	 * Add seed, encrypt it, and create default accounts
	 */
	fun addSeed(seedName: String, seedPhrase: String): String {

		//Check if seed name already exists
		if (seedNames.value?.contains(seedName) as Boolean) {
			_lastError.value = "Seed with this name already exists!"
			return ""
		}

		try {
			//Run standard login prompt!
			authentication.authenticate(context as FragmentActivity) { /*TODO*/  }

			//Create relevant keys - should make sure this works before saving key
			val finalSeedPhrase = substrateTryCreateSeed(seedName, "sr25519", seedPhrase, 24, dbName)

			//Encrypt and save seed

			Log.d("Create seed", "success")
			refreshSeedNames()
			selectSeed(seedName)
			return finalSeedPhrase
		} catch (e: java.lang.Exception) {
			_lastError.value = e.toString()
			Log.e("Seed creation error", e.toString())
			return ""
		}
	}

	fun selectSeed(seedName: String) {

	}

	//MARK: Seed management end

	//MARK: Network management begin

	/**
	 * Get network list updated; call after any networks-altering operation
	 * and on init and on refresh just in case
	 */
	fun refreshNetworks() {
		val networkJSON = dbGetAllNetworksForNetworkSelector(dbName)
		try {
			_networks.value = JSONArray(networkJSON)
		} catch (e: java.lang.Exception) {
			Log.e("Refresh network error", e.toString())
		}
	}


	fun selectNetwork(network: JSONObject) {
		_selectedNetwork.value = network
	}

	//MARK: Network management end

	//MARK: Key management begin



	//MARK: Key management end

	//MARK: rust native section begin

	external fun substrateExportPubkey(address: String, network: String, dbname: String): String
	external fun qrparserGetPacketsTotal(data: String): Int
	external fun qrparserTryDecodeQrSequence(data: String): String
	external fun substrateParseTransaction(transaction: String, dbname: String): String
	external fun substrateHandleAction(action: String, seedPhrase: String, password: String, dbname: String): String
	external fun substrateDevelopmentTest(input: String): String
	external fun dbGetNetwork(genesisHash: String, dbname: String): String
	external fun dbGetAllNetworksForNetworkSelector(dbname: String): String
	external fun dbGetRelevantIdentities(seedName: String, genesisHash: String, dbname: String): String
	external fun dbGetAllIdentities(dbname: String): String
	external fun substrateTryCreateSeed(seedName: String, crypto: String, seedPhrase: String, seedLength: Int, dbname: String): String
	external fun substrateSuggestNPlusOne(path: String, seedName: String, networkIdString: String, dbname: String): String
	external fun substrateCheckPath(path: String): Boolean
	external fun substrateTryCreateIdentity(idName: String, seedName: String, seedPhrase: String, crypto: String, path: String, network: String, hasPassword: Boolean, dbname: String)
	external fun substrateSuggestName(path: String): String
	external fun substrateDeleteIdentity(pubKey: String, network: String, dbname: String)
	external fun substrateGetNetworkSpecs(network: String, dbname: String): String
	external fun substrateRemoveNetwork(network: String, dbname: String)
	external fun substrateRemoveMetadata(networkName: String, networkVersion: Int, dbname: String)
	external fun substrateRemoveSeed(seedName: String, dbname: String)

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
