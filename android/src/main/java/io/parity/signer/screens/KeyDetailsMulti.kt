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
import androidx.compose.ui.Modifier
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.KeyCardOld
import io.parity.signer.components.NetworkCard
import io.parity.signer.components.NetworkCardModel
import io.parity.signer.models.intoImageBitmap
import io.parity.signer.ui.theme.Bg200
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.MAddressCard
import io.parity.signer.uniffi.MKeyDetailsMulti

/**
 * This screen is replaced by animated qr bottom sheet
 * todo remove it
 */
@Composable
fun KeyDetailsMulti(
	keyDetailsMulti: MKeyDetailsMulti,
	button: (Action) -> Unit
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
		) {
			KeyCardOld(identity = MAddressCard(
				address = keyDetailsMulti.keyDetails.address,
				base58 = keyDetailsMulti.keyDetails.base58,
				multiselect = keyDetailsMulti.keyDetails.multiselect
			))
		}
		Row(
			Modifier.padding(top = 3.dp, start = 12.dp, end = 12.dp)
		) {
			NetworkCard(
				NetworkCardModel(
					keyDetailsMulti.keyDetails.networkInfo.networkTitle,
					keyDetailsMulti.keyDetails.networkInfo.networkLogo,
				)
			)
		}
		Image(
			(keyDetailsMulti.keyDetails.qr).intoImageBitmap(),
			contentDescription = stringResource(id = R.string.qr_with_address_to_scan_description),
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
							button(Action.NEXT_UNIT)
						} else {
							if (offset > 100) {
								button(Action.PREVIOUS_UNIT)
							}
						}
						offset = 0f
					}
				)
		)
		Text(
			"Key " + keyDetailsMulti.currentNumber + " out of " + keyDetailsMulti.outOf
		)
	}
}
