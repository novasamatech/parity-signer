package io.parity.signer.models

import android.content.Context
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import io.parity.signer.MainActivity

/**
 * This is single object to handle all interactions with backend,
 * except for some logging features and transaction handling
 */
class SignerDataModel: ViewModel() {
	private val _onBoardingDone = MutableLiveData(false)
	private val _developmentTest = MutableLiveData("")

	val onBoardingDone: LiveData<Boolean> = _onBoardingDone
	val developmentTest: LiveData<String> = _developmentTest

	lateinit var context: Context

	//actually load RustNative code
	init {
		System.loadLibrary("signer")
	}

	/**
	 * This returns the app into starting state; should be called
	 * on all "back"-like events
	 */
	fun totalRefresh() {
		_onBoardingDone.value = true
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
