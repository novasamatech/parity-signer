package io.parity.signer.models

import android.app.Activity
import android.app.PendingIntent.getActivity
import android.content.Context
import android.content.SharedPreferences
import android.security.keystore.KeyGenParameterSpec
import android.security.keystore.KeyProperties
import android.util.Log
import androidx.fragment.app.FragmentActivity
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import androidx.security.crypto.EncryptedSharedPreferences
import androidx.security.crypto.MasterKeys
import io.parity.signer.MainActivity
import io.parity.signer.components.Authentication
import org.json.JSONArray
import org.json.JSONObject
import java.io.File
import java.io.FileOutputStream
import java.io.IOException
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

	//Data storage locations
	private var dbName: String = ""
	private val keyStore = "SignerKeyStore"
	private var masterKey = "SignerMasterKey"
	private lateinit var sharedPreferences: SharedPreferences

	//Observables
	val onBoardingDone: LiveData<Boolean> = _onBoardingDone
	val developmentTest: LiveData<String> = _developmentTest
	val networks: LiveData<JSONArray> = _networks
	val selectedNetwork: LiveData<JSONObject> = _selectedNetwork

	lateinit var context: Context
	lateinit var activity: FragmentActivity
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
		totalRefresh()
	}

	/**
	 * Populate database!
	 */
	fun onBoard() {
		copyAsset("")

		//Init crypto for seeds:
		//1. Generate key
		val keyGen = KeyGenerator
			.getInstance(KeyProperties.KEY_ALGORITHM_AES, keyStore)
		keyGen.init(
				KeyGenParameterSpec
					.Builder(
						masterKey,
						KeyProperties.PURPOSE_ENCRYPT and KeyProperties.PURPOSE_DECRYPT
					)
					.setBlockModes(KeyProperties.BLOCK_MODE_CBC)
					.setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_PKCS7)
					.setUserAuthenticationParameters(0, KeyProperties.AUTH_DEVICE_CREDENTIAL)
					.setUserAuthenticationRequired(true)
					.build()
			)
		keyGen.generateKey()

		/*
		//2. Generate storage
		sharedPreferences = EncryptedSharedPreferences.create(
			keyStore,
			masterKey,
			context,
			EncryptedSharedPreferences.PrefKeyEncryptionScheme.AES256_SIV,
			EncryptedSharedPreferences.PrefValueEncryptionScheme.AES256_GCM
		)
*/
		totalRefresh()
	}

	/**
	 * Util to copy single Assets file
	 */
	private fun copyFileAsset(path: String) {
		var file = File(dbName, path)
		file.createNewFile()
		var input = context.assets.open("Database" + path)
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
		val contents = context.assets.list("Database" + path)
		if (contents == null || contents.size == 0) {
			copyFileAsset(path)
		} else {
			File(dbName, path).mkdirs()
			for (entry in contents) copyAsset(path + "/" + entry)
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

	//MARK: Network management begin

	/**
	 * Get network list updated; call after any networks-altering operation
	 * and on init and on refresh just in case
	 */
	fun refreshNetworks() {
		val networkJSON = dbGetAllNetworksForNetworkSelector(dbName)
		try {
			_networks.value = JSONArray(networkJSON)
			Log.d("reftesh", "happened")
			Log.d("testval", networks.value.toString())
		} catch (e: java.lang.Exception) {
			Log.d("errormark", "happened")
			Log.d("catcher", e.toString())
		}
	}


	fun selectNetwork(network: JSONObject) {
		_selectedNetwork.value = network
	}

	//MARK: Network management end

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
	external fun substrateTryCreateSeed(seedName: String, crypto: String, seedPhrase: String, seedLength: Int, dbname: Int): String
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
