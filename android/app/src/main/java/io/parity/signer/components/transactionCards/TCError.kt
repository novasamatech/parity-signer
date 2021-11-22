package io.parity.signer.components.transactionCards

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Row
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier

@Composable
fun TCError(error: String) {
	Row(
		modifier = Modifier.background(MaterialTheme.colors.error)
	) {
		Text("Error! ", color = MaterialTheme.colors.onError)
		Text(error, color = MaterialTheme.colors.onError)
	}
}
