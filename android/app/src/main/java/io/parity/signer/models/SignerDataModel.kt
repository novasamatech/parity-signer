package io.parity.signer.models

import android.app.Activity
import android.app.PendingIntent.getActivity
import android.content.Context
import android.util.Log
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import io.parity.signer.MainActivity
import java.io.File
import java.io.FileOutputStream
import java.io.IOException

/**
 * This is single object to handle all interactions with backend,
 * except for some logging features and transaction handling
 */
class SignerDataModel: ViewModel() {
	private val _onBoardingDone = MutableLiveData(false)
	private val _developmentTest = MutableLiveData("")

	private var dbName: String = ""

	val onBoardingDone: LiveData<Boolean> = _onBoardingDone
	val developmentTest: LiveData<String> = _developmentTest

	lateinit var context: Context

	init {
		//actually load RustNative code
		System.loadLibrary("signer")
	}

	fun lateInit() {
		//Define local database name
		dbName = context.applicationContext.filesDir.toString() + "/Database"
		totalRefresh()
	}

	/**
	 * Populate database!
	 */
	fun onBoard() {
		copyAsset("")
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
		val buffer: ByteArray = ByteArray(1024)
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

	/**
	 * This returns the app into starting state; should be called
	 * on all "back"-like events and new screen spawns just in case
	 */
	fun totalRefresh() {
		_onBoardingDone.value = File(dbName).exists()
	}

	fun isOnBoardingDone(): Boolean {
		return File(dbName).exists()
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

	//rust native section begin

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

	//rust native section end

}
