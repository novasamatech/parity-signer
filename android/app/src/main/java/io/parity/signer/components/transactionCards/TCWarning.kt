package io.parity.signer.components.transactionCards

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Row
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier

@Composable
fun TCWarning(warning: String) {
	Row(
		modifier = Modifier.background(MaterialTheme.colors.error)
	) {
		Text("Warning! ", color = MaterialTheme.colors.onError)
		Text(warning, color = MaterialTheme.colors.onError)
	}
}
