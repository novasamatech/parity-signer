package io.parity.signer.components

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.AirplanemodeActive
import androidx.compose.material.icons.filled.WifiOff
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.Bg200
import io.parity.signer.ui.theme.Text300
import io.parity.signer.ui.theme.Typography
import io.parity.signer.ui.theme.Text600

@Composable
fun InstructionsSquare() {
	Surface (
		color = Bg200,
		shape = MaterialTheme.shapes.large
		) {
		Column(Modifier.padding(16.dp)) {
			Icon(Icons.Default.AirplanemodeActive, "Airplane mode enabled")
			Text(
				"Use Signer in Airplane mode",
				style = Typography.body2,
				color = Text600
			)
			Text(
				"Airplane mode will stop your phone from using mobile data. Signer will only work when you have no wifi and no mobile connection!",
				style = Typography.subtitle2,
				color = Text300
			)
			Icon(Icons.Default.WifiOff, "All interfaces should be disabled")
			Text("Airgap your phone", style = Typography.body2, color = Text600)
			Text(
				"Make sure your phone's Bluetooth, NFC and other sensors are off, and that all cables are disconnected. Signer will not check these connections, so it is important that you do!",
				style = Typography.subtitle2,
				color = Text300
			)
		}
	}
}
