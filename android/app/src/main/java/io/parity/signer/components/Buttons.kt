package io.parity.signer.components

import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.*

@Composable
fun BigButton(
	text: String,
	isShaded: Boolean = false,
	isCrypto: Boolean = false,
	isDangerous: Boolean = false,
	action: () -> Unit,
	isDisabled: Boolean = false
) {
	val accentColor = if (isCrypto) MaterialTheme.colors.Crypto400 else MaterialTheme.colors.Action400
	val bgColor = if (isDisabled) MaterialTheme.colors.Bg200 else if (isShaded) MaterialTheme.colors.Bg300 else accentColor
	val fgColor =
		if (isDisabled) MaterialTheme.colors.Text300 else if (isShaded) if (isDangerous) MaterialTheme.colors.SignalDanger else accentColor else MaterialTheme.colors.Action600

	Button(
		onClick = action,
		enabled = !isDisabled,
		colors = ButtonDefaults.buttonColors(backgroundColor = bgColor),
		modifier = Modifier.padding(8.dp)
	) {
		Row(
			horizontalArrangement = Arrangement.Center,
		) {
			Spacer(Modifier.weight(1f))
			Text(text, style = MaterialTheme.typography.button, color = fgColor)
			Spacer(Modifier.weight(1f))
		}
	}
}
