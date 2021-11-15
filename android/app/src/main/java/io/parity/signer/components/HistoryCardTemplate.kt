package io.parity.signer.components

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.BaseDanger
import io.parity.signer.ui.theme.Crypto400
import io.parity.signer.ui.theme.Text300

@Composable
fun HistoryCardTemplate(
	image: ImageVector,
	line1: String,
	line2: String,
	line3: String,
	danger: Boolean = false
) {
	val color2 = if (danger) {
		BaseDanger
	} else {
		Crypto400
	}
	val color3 = Text300

	Row(
		verticalAlignment = Alignment.CenterVertically,
		modifier = Modifier
			.padding(8.dp)
			.fillMaxWidth(1f)
	) {
		Icon(image, "history event icon", modifier = Modifier, tint = color2)
		Column(
			modifier = Modifier.padding(horizontal = 8.dp)
		) {
			Text(line1.substring(0, 16), style = MaterialTheme.typography.overline)
			Text(line2, color = color2, style = MaterialTheme.typography.overline)
			if (line3.isNotBlank()) Text(
				line3,
				color = color3,
				style = MaterialTheme.typography.overline
			)
		}
	}
}
