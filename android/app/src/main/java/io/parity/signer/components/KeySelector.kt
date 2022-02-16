package io.parity.signer.components

import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import io.parity.signer.ButtonID
import io.parity.signer.models.toListOfJSONObjects
import org.json.JSONArray
import org.json.JSONObject

@Composable
fun KeySelector(
	button: (button: ButtonID, details: String) -> Unit,
	increment: (Int) -> Unit,
	keySet: JSONArray,
	multiSelectMode: Boolean
) {
	val addresses = keySet.toListOfJSONObjects() ?: emptyList()
	LazyColumn {
		this.items(
			items = addresses,
			key = {
				it.optString("address_key")
			}
		) { address ->
			val addressKey = address.optString("address_key")
			val selectButton = {
				button(
					ButtonID.SelectKey,
					addressKey
				)
			}
			val longTapButton = {
				button(
					ButtonID.LongTap,
					addressKey
				)
			}
			val swipe = { button(ButtonID.Swipe, addressKey) }
			val delete = { button(ButtonID.RemoveKey, "") }
			KeyCardActive(address, selectButton, longTapButton, swipe, increment, delete, multiSelectMode)
		}
	}
}
