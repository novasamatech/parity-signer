package io.parity.signer.components

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.height
import androidx.compose.material.Divider
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.Text400
import io.parity.signer.ui.theme.Text600

/**
 * 2-line header bar
 */
@Composable
fun HeaderBar(
	line1: String,
	line2: String,
	modifier: Modifier = Modifier
) {
	Column(modifier = modifier) {
		HeadingOverline(text = line1)
		Text(
			line2,
			style = MaterialTheme.typography.subtitle2,
			color = MaterialTheme.colors.Text400
		)
		Divider()
		Spacer(Modifier.height(12.dp))
	}
}

@Composable
fun HeadingOverline(text: String) {
	Text(
		text.uppercase(),
		style = MaterialTheme.typography.overline,
		color = MaterialTheme.colors.Text600
	)
}

