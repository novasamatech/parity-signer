package io.parity.signer.screens.settings.logs.logdetails

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.*

@Composable
fun HistoryCardTemplateOld(
	image: ImageVector,
	line1: String,
	line2: String,
	line3: String,
	danger: Boolean = false
) {
	val color1 = MaterialTheme.colors.Text500
	val color2 = if (danger) {
		MaterialTheme.colors.SignalDanger
	} else {
		MaterialTheme.colors.Crypto400
	}
	val color3 = MaterialTheme.colors.Text300
	val line1cut = if (line1.length>16) line1.substring(0, 16) else line1

	Row(
		verticalAlignment = Alignment.CenterVertically,
		modifier = Modifier
			.padding(top = 3.dp, start = 12.dp, end = 12.dp)
			.fillMaxWidth(1f)
			.background(MaterialTheme.colors.Bg200)
			.padding(8.dp)
	) {
		Column(
			modifier = Modifier.padding(horizontal = 8.dp)
		) {
			Text(line1cut, color = color1, style = MaterialTheme.typography.subtitle2)
			Text(line2, color = color2, style = MaterialTheme.typography.subtitle2)
			if (line3.isNotBlank()) Text(
				line3,
				color = color3,
				style = CryptoTypography.body2
			)
		}
		Spacer(Modifier.weight(1f))
		Icon(image, "history event icon", modifier = Modifier, tint = color2)
	}
}
