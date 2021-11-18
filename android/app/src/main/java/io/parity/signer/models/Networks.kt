package io.parity.signer.models

import android.util.Log
import org.json.JSONArray
import org.json.JSONObject

/**
 * This is how hard types should look like; just copy from ios I suppose?
 */
data class Network(
	val key: String,
	val color: String,
	val logo: String,
	val order: String,
	val secondaryColor: String,
	val title: String
)

/**
 * Get network list updated; call after any networks-altering operation
 * and on init and on refresh just in case
 */
internal fun SignerDataModel.refreshNetworks() {
	try {
		val networkJSON = dbGetAllNetworksForNetworkSelector(dbName)
		_networks.value = JSONArray(networkJSON)
		fetchKeys()
	} catch (e: java.lang.Exception) {
		Log.e("Refresh network error", e.toString())
	}
}

fun SignerDataModel.selectNetwork(network: JSONObject) {
	_selectedNetwork.value = network
	fetchKeys()
}

