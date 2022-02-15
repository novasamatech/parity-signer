package io.parity.signer.components.transactionCards

import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.width
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.Text400
import io.parity.signer.ui.theme.Text600

/**
 * This is not a ready card, but a template for typical card with 2 fields
 */
@Composable
fun TCNameValueTemplate(name: String, value: String) {
	Row {
		Text(name, style = MaterialTheme.typography.body2, color = MaterialTheme.colors.Text400)
		Spacer(Modifier.width(16.dp))
		Text(value, style = MaterialTheme.typography.body2, color = MaterialTheme.colors.Text600)
	}
}
