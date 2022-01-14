package io.parity.signer.components

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
fun HistoryCardTemplate(
	image: ImageVector,
	line1: String,
	line2: String,
	line3: String,
	danger: Boolean = false
) {
	val color1 = Text500
	val color2 = if (danger) {
		BaseDanger
	} else {
		Crypto400
	}
	val color3 = Text300

	Row(
		verticalAlignment = Alignment.CenterVertically,
		modifier = Modifier
			.padding(top = 3.dp, start = 12.dp, end = 12.dp)
			.fillMaxWidth(1f)
			.background(Bg200)
			.padding(8.dp)
	) {
		Column(
			modifier = Modifier.padding(horizontal = 8.dp)
		) {
			Text(line1.substring(0, 16), color = color1, style = MaterialTheme.typography.subtitle2)
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
