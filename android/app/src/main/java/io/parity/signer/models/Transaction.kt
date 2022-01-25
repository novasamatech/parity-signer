package io.parity.signer.models

import android.util.Log
import androidx.compose.ui.graphics.ImageBitmap
import org.json.JSONArray
import org.json.JSONObject

/**
 * Turn backend payload section into nice sorted array of transaction cards
 */
fun JSONObject.parseTransaction(): JSONArray {
	val warnings = this.optJSONArray("warning") ?: JSONArray()
	val error = (this.optJSONArray("error") ?: JSONArray())
	val typesInfo =
		this.optJSONArray("types_info") ?: JSONArray()
	val method = (this.optJSONArray("method") ?: JSONArray())
	val extensions =
		(this.optJSONArray("extensions") ?: JSONArray())
	val newSpecs = (this.optJSONArray("new_specs") ?: JSONArray())
	val verifier = (this.optJSONArray("verifier") ?: JSONArray())
	return sortCards(
		concatJSONArray(
			warnings,
			error,
			typesInfo,
			method,
			extensions,
			newSpecs,
			verifier
		)
	)
}

enum class TransactionType {
	sign,
	stub,
	read,
	import_derivations,
	done;
}
