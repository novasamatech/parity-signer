package io.parity.signer.components

import androidx.compose.foundation.layout.Column
import androidx.compose.material.Divider
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier

@Composable
fun HeaderBar(
	line1: String,
	line2: String,
	modifier: Modifier = Modifier
) {
	Column(modifier = modifier) {
		Text(line1, style = MaterialTheme.typography.overline)
		Text(line2, style = MaterialTheme.typography.subtitle2)
		Divider()
	}
}
