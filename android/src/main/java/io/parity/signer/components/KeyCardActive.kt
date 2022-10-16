package io.parity.signer.components

import androidx.compose.foundation.background
import androidx.compose.foundation.gestures.Orientation
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.gestures.draggable
import androidx.compose.foundation.gestures.rememberDraggableState
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.Bg200
import io.parity.signer.uniffi.Address
import io.parity.signer.uniffi.MAddressCard
import io.parity.signer.uniffi.MKeysCard
import kotlin.math.absoluteValue

@Composable
fun KeyCardActive(
	keys_card: MKeysCard,
	rootSeed: String,
	selectButton: () -> Unit,
	longTapButton: () -> Unit,
	swipe: () -> Unit,
	increment: (Int) -> Unit,
	delete: () -> Unit,
	multiselectMode: Boolean
) {
	var offsetX by remember { mutableStateOf(0f) }

	Row(
		verticalAlignment = Alignment.CenterVertically,
		modifier = Modifier
			.padding(top = 3.dp, start = 12.dp, end = 12.dp)
			.background(MaterialTheme.colors.Bg200)
	) {
		Row(
			verticalAlignment = Alignment.CenterVertically,
			modifier = Modifier
				.pointerInput(Unit) {
					detectTapGestures(
						onTap = {
							selectButton()
						},
						onLongPress = {
							longTapButton()
						}
					)
				}
				.draggable(
					state = rememberDraggableState { delta ->
						offsetX += delta
					},
					orientation = Orientation.Horizontal,
					onDragStopped = {
						if (offsetX.absoluteValue > 20f) {
							swipe()
						}
						offsetX = 0f
					}
				)
		) {
			KeyCardOld(
				identity = MAddressCard(
					base58 = keys_card.base58,
					multiselect = keys_card.multiselect,
					address = Address(
					path = keys_card.address.path,
					hasPwd = keys_card.address.hasPwd,
					identicon = keys_card.address.identicon,
					secretExposed = keys_card.address.secretExposed,
					seedName = rootSeed)
				), multiselectMode
			)
			Spacer(modifier = Modifier.weight(1f, true))
			if (keys_card.swiped) {
				SwipedButtons(increment, delete)
			}
		}
	}
}
