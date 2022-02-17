package io.parity.signer.components

import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.ButtonID
import io.parity.signer.models.toListOfJSONObjects
import org.json.JSONArray

@Composable
fun KeySelector(
	button: (button: ButtonID, details: String) -> Unit,
	increment: (Int) -> Unit,
	keySet: JSONArray,
	multiSelectMode: Boolean
) {
	val addresses = keySet.toListOfJSONObjects().sortedBy { it.optString("path") }
	LazyColumn {
		this.items(
			items = addresses,
			key = {
				it.optString("address_key")
			}
		) { address ->
			val addressKey = address.optString("address_key")
			KeyCardActive(
				address,
				selectButton = { button(ButtonID.SelectKey, addressKey) },
				longTapButton = { button(ButtonID.LongTap, addressKey) },
				swipe = { button(ButtonID.Swipe, addressKey) },
				increment,
				delete = { button(ButtonID.RemoveKey, "") },
				multiSelectMode
			)
		}
		item{
			Spacer(Modifier.height(60.dp))
		}
	}
}
