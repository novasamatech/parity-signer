package io.parity.signer.components

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ChevronRight
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.Bg200
import io.parity.signer.ui.theme.Border400
import io.parity.signer.ui.theme.SignalDanger
import io.parity.signer.ui.theme.Text400

@Composable
fun SettingsCardTemplate(
	text: String,
	danger: Boolean = false,
	withIcon: Boolean = true,
	withBackground: Boolean = true
) {
	Row(
		verticalAlignment = Alignment.CenterVertically,
		modifier = Modifier
			.background(
				color = if (withBackground) {
					MaterialTheme.colors.Bg200
				} else {
					Color(0x00000000)
				}
			)
	) {
		Text(
			text, style = MaterialTheme.typography.body1, color = if (danger) {
				MaterialTheme.colors.SignalDanger
			} else {
				MaterialTheme.colors.Text400
			}, modifier = Modifier.padding(12.dp)
		)
		Spacer(Modifier.weight(1f))
		if (withIcon) {
			Icon(
				Icons.Default.ChevronRight,
				"go forward",
				tint = MaterialTheme.colors.Border400,
				modifier = Modifier.padding(16.dp)
			)
		}
	}
}
