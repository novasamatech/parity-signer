package io.parity.signer.screens

import androidx.compose.foundation.Image
import androidx.compose.foundation.gestures.Orientation
import androidx.compose.foundation.gestures.draggable
import androidx.compose.foundation.gestures.rememberDraggableState
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.offset
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
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
import org.json.JSONObject

@Composable
fun KeyDetailsMulti(signerDataModel: SignerDataModel) {
	val address = signerDataModel.screenData.observeAsState()
	var offset by remember { mutableStateOf(0f) }

	Column(
		modifier = Modifier
			.fillMaxWidth()
			.verticalScroll(rememberScrollState())
	) {
		KeyCard(identity = address.value?: JSONObject())
		NetworkCard(address.value?: JSONObject())
		Image(
			(address.value?.optString("qr")?:"").intoImageBitmap(),
			contentDescription = "QR with address to scan",
			contentScale = ContentScale.FillWidth,
			modifier = Modifier
				.fillMaxWidth(1f)
				.offset(x = offset.dp)
				.draggable(
					orientation = Orientation.Horizontal,
					state = rememberDraggableState { delta ->
						offset += delta
					},
					onDragStopped = {
						if (offset < -100) {
							signerDataModel.pushButton(ButtonID.NextUnit)
						} else {
							if (offset > 100) {
								signerDataModel.pushButton(ButtonID.PreviousUnit)
							}
						}
						offset = 0f
					}
				)
		)
		Text(
			"Key " + address.value?.optString("current_number") + " out of " + address.value?.optString(
				"out_of"
			)
		)
	}
}
