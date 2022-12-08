package io.parity.signer.components.transactionCards

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Row
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import io.parity.signer.ui.theme.BgDanger
import io.parity.signer.ui.theme.SignalDanger

@Composable
fun TCError(error: String) {
	Row(
		modifier = Modifier.background(MaterialTheme.colors.BgDanger)
	) {
		Text("Error! ", color = MaterialTheme.colors.SignalDanger)
		Text(error, color = MaterialTheme.colors.SignalDanger)
	}
}
