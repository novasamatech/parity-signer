package io.parity.signer.components

import android.util.Log
import androidx.compose.foundation.background
import androidx.compose.foundation.gestures.*
import androidx.compose.foundation.gestures.draggable
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.runtime.mutableStateOf
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.Bg200
import org.json.JSONObject
import kotlin.math.absoluteValue

@Composable
fun KeyCardActive(
	address: JSONObject,
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
				.padding(horizontal = 8.dp)
		) {
			KeyCard(
				address, multiselectMode
			)
			Spacer(modifier = Modifier.weight(1f, true))
			if (address.optBoolean("swiped")) {
				SwipedButtons(increment, delete)
			}
		}
	}
}
