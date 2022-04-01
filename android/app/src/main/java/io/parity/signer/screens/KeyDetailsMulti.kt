package io.parity.signer.screens

import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.gestures.Orientation
import androidx.compose.foundation.gestures.draggable
import androidx.compose.foundation.gestures.rememberDraggableState
import androidx.compose.foundation.layout.*
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.unit.dp
import io.parity.signer.ButtonID
import io.parity.signer.components.KeyCard
import io.parity.signer.components.NetworkCard
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.intoImageBitmap
import io.parity.signer.models.pushButton
import io.parity.signer.ui.theme.Bg200
import org.json.JSONObject

@Composable
fun KeyDetailsMulti(
	screenData: JSONObject,
	button: (button: ButtonID) -> Unit
) {
	var offset by remember { mutableStateOf(0f) }

	Column(
		modifier = Modifier
			.fillMaxWidth()
	) {
		Row(
			Modifier
				.padding(top = 3.dp, start = 12.dp, end = 12.dp)
				.background(
					MaterialTheme.colors.Bg200
				)
				.fillMaxWidth()
		) { KeyCard(identity = screenData) }
		Row(
			Modifier.padding(top = 3.dp, start = 12.dp, end = 12.dp)
		) {
			NetworkCard(screenData)
		}
		Image(
			(screenData.optString("qr")).intoImageBitmap(),
			contentDescription = "QR with address to scan",
			contentScale = ContentScale.FillWidth,
			modifier = Modifier
				.fillMaxWidth(1f)
				.padding(12.dp)
				.offset(x = offset.dp)
				.draggable(
					orientation = Orientation.Horizontal,
					state = rememberDraggableState { delta ->
						offset += delta
					},
					onDragStopped = {
						if (offset < -100) {
							button(ButtonID.NextUnit)
						} else {
							if (offset > 100) {
								button(ButtonID.PreviousUnit)
							}
						}
						offset = 0f
					}
				)
		)
		Text(
			"Key "
				+ screenData.optString("current_number")
				+ " out of "
				+ screenData.optString("out_of")
		)
	}
}
