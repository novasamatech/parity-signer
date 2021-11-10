package io.parity.signer.components

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.Icon
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.unit.dp

@Composable
fun HistoryCardTemplate(image: ImageVector, line1: String, line2: String, line3: String, danger: Boolean = false) {
	Row (
		verticalAlignment = Alignment.CenterVertically,
		modifier = Modifier.padding(8.dp).fillMaxWidth(1f)
		) {
		Icon(image, "history event icon")
		Column (
			modifier = Modifier.padding(horizontal = 8.dp)
			) {
			Text(line1.substring(0, 16))
			Text(line2)
			if (line3.isNotBlank()) Text(line3)
		}
	}
}
