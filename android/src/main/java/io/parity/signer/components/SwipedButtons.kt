package io.parity.signer.components

import androidx.compose.foundation.clickable
import androidx.compose.foundation.gestures.Orientation
import androidx.compose.foundation.gestures.draggable
import androidx.compose.foundation.gestures.rememberDraggableState
import androidx.compose.foundation.layout.*
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Delete
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.alerts.AndroidCalledConfirm
import io.parity.signer.ui.theme.Crypto100
import io.parity.signer.ui.theme.Crypto400
import io.parity.signer.ui.theme.CryptoTypography
import io.parity.signer.ui.theme.SignalDanger
import kotlin.math.absoluteValue
import kotlin.math.exp

@Composable
fun SwipedButtons(
	increment: (Int) -> Unit,
	delete: () -> Unit
) {
	var offsetY by remember { mutableStateOf(0f) }
	var confirm by remember { mutableStateOf(false) }

	Row {
		Surface(
			color = MaterialTheme.colors.Crypto100,
			shape = MaterialTheme.shapes.large,
			modifier = Modifier
				.draggable(
					state = rememberDraggableState { delta ->
						offsetY += delta
					},
					orientation = Orientation.Vertical,
					onDragStopped = {
						increment(exp(offsetY.absoluteValue / 500f).toInt())
						offsetY = 0f
					}
				)
				.clickable { increment(1) }
				.size(39.dp)
		) {
			Box(contentAlignment = Alignment.Center) {
				Text(
					"N+" + exp(offsetY.absoluteValue / 500f).toInt().toString(),
					style = CryptoTypography.body2,
					color = MaterialTheme.colors.Crypto400
				)
			}
		}
		Spacer(Modifier.width(20.dp))
		Surface(
			color = MaterialTheme.colors.SignalDanger,
			shape = MaterialTheme.shapes.large,
			modifier = Modifier
				.clickable { confirm = true }
				.size(39.dp)
		) {
			Box(contentAlignment = Alignment.Center) {
				Icon(Icons.Default.Delete, "remove key")
			}
		}
	}

	AndroidCalledConfirm(
		show = confirm,
		header = "Delete key?",
		text = "You are about to delete key",
		back = { confirm = false },
		forward = { delete() })
}
