package io.parity.signer.components

import android.util.Log
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.GppBad
import androidx.compose.material.icons.filled.GppGood
import androidx.compose.runtime.Composable
import io.parity.signer.ui.theme.Crypto400
import io.parity.signer.ui.theme.SignalDanger
import io.parity.signer.ui.theme.SignalWarning
import io.parity.signer.uniffi.ShieldAlert

@Composable
fun NavbarShield(alert: ShieldAlert?, active: Boolean) {
	alert?.let {
		Log.w("SIGNER_RUST_LOG", "alert $alert")
	}
	if (active) {
		Icon(
			Icons.Default.GppBad,
			"device is online",
			tint = MaterialTheme.colors.SignalDanger
		)
	} else {
		when (alert) {
			null -> Icon(
				Icons.Default.GppGood,
				"device is safe",
				tint = MaterialTheme.colors.Crypto400
			)
			ShieldAlert.PAST -> Icon(
				Icons.Default.GppBad,
				"potential security breach",
				tint = MaterialTheme.colors.SignalWarning
			)
		}
	}
}
