package io.parity.signer.components

import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.MKeysCard

@Composable
fun KeySelector(
	button: (action: Action, details: String) -> Unit,
	increment: (Int) -> Unit,
	keySet: List<MKeysCard>,
	multiSelectMode: Boolean,
	rootSeed: String,
) {
	val addresses = keySet.sortedBy { it.path }
	LazyColumn {
		this.items(
			items = addresses,
			key = {
				it.addressKey
			}
		) { address ->
			val addressKey = address.addressKey
			KeyCardActive(
				address,
				rootSeed = rootSeed,
				selectButton = { button(Action.SELECT_KEY, addressKey) },
				longTapButton = { button(Action.LONG_TAP, addressKey) },
				swipe = { button(Action.SWIPE, addressKey) },
				increment,
				delete = { button(Action.REMOVE_KEY, "") },
				multiSelectMode
			)
		}
		item{
			Spacer(Modifier.height(60.dp))
		}
	}
}
