package io.parity.signer.components

import androidx.compose.foundation.background
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
	val accentColor = if (isCrypto) Crypto400 else Action400
	val bgColor = if (isDisabled) Bg200 else if (isShaded) Bg300 else accentColor
	val fgColor =
		if (isDisabled) Text300 else if (isShaded) if (isDangerous) BaseDanger else accentColor else Action600

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
