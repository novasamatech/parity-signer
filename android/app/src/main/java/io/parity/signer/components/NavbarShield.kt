package io.parity.signer.components

import android.util.Log
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.GppBad
import androidx.compose.material.icons.filled.GppGood
import androidx.compose.material.icons.filled.GppMaybe
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.runtime.livedata.observeAsState
import io.parity.signer.models.SignerDataModel
import io.parity.signer.ui.theme.Crypto400
import io.parity.signer.ui.theme.SignalDanger
import io.parity.signer.ui.theme.SignalWarning
import io.parity.signer.uniffi.ShieldAlert

@Composable
fun NavbarShield(alert: ShieldAlert?) {
	alert?.let {
		Log.w("SIGNER_RUST_LOG", "alert $alert")
	}
	when (alert) {
		null -> Icon(
			Icons.Default.GppGood,
			"device is safe",
			tint = MaterialTheme.colors.Crypto400
		)
		ShieldAlert.ACTIVE -> Icon(
			Icons.Default.GppBad,
			"device is online",
			tint = MaterialTheme.colors.SignalDanger
		)
		ShieldAlert.PAST -> Icon(
			Icons.Default.GppBad,
			"potential security breach",
			tint = MaterialTheme.colors.SignalWarning
		)
	}
}
