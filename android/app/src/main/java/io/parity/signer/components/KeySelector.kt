package io.parity.signer.components

import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import io.parity.signer.ButtonID
import io.parity.signer.models.toListOfJSONObjects
import org.json.JSONObject

@Composable
fun KeySelector(
	button: (button: ButtonID, details: String) -> Unit,
	screenData: JSONObject
) {
	val addresses = screenData.optJSONArray("set")?.toListOfJSONObjects() ?: emptyList()
	LazyColumn {
		this.items(
			items = addresses,
			key = {
				it.optString("address_key")
			}
		) { address ->
			val addressKey = address.optString("address_key")
			val selectButton by remember { mutableStateOf(
				{
				button(
					ButtonID.SelectKey,
					addressKey
				)
				}
			)
			}
			val longTapButton = {
				button(
					ButtonID.LongTap,
					addressKey
				)
			}
			KeyCardActive(address, selectButton, longTapButton)
		}
	}
}
