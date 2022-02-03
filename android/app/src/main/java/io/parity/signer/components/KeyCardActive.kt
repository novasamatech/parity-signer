package io.parity.signer.components

import androidx.compose.foundation.background
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.CheckCircle
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.Bg200
import org.json.JSONObject

@Composable
fun KeyCardActive(
	address: JSONObject,
	selectButton: () -> Unit,
	longTapButton: () -> Unit,
) {
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
				.padding(horizontal = 8.dp)
		) {
			KeyCard(
				address
			)
			Spacer(modifier = Modifier.weight(1f, true))
			Icon(Icons.Default.CheckCircle, "Address selected")
		}
	}
}
