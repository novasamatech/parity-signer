package io.parity.signer.components

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.width
import androidx.compose.material.Button
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
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
	val fgColor = if (isDisabled) Text300 else if (isShaded) if (isDangerous) BaseDanger else accentColor else Action600

	Button(onClick = action, enabled = !isDisabled) {
		Row (
			horizontalArrangement = Arrangement.Center,
			modifier = Modifier.background(bgColor)
			) {
			Spacer(Modifier.weight(1f))
			Text(text, style = MaterialTheme.typography.button, color = fgColor)
			Spacer(Modifier.weight(1f))
		}
	}
}
