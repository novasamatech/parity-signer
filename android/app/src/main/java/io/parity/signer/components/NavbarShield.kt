package io.parity.signer.components

import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.GppBad
import androidx.compose.material.icons.filled.GppGood
import androidx.compose.material.icons.filled.GppMaybe
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import io.parity.signer.ShieldAlert
import io.parity.signer.models.SignerDataModel
import io.parity.signer.ui.theme.Crypto400
import io.parity.signer.ui.theme.SignalDanger
import io.parity.signer.ui.theme.SignalWarning

@Composable
fun NavbarShield(signerDataModel: SignerDataModel) {
	val alert = signerDataModel.alertState.observeAsState()

	when(alert.value) {
		ShieldAlert.None -> Icon(Icons.Default.GppGood, "device is safe", tint = MaterialTheme.colors.Crypto400)
		ShieldAlert.Active -> Icon(Icons.Default.GppBad, "device is online", tint = MaterialTheme.colors.SignalDanger)
		ShieldAlert.Past -> Icon(Icons.Default.GppBad, "potential security breach", tint = MaterialTheme.colors.SignalWarning)
		null -> Icon(Icons.Default.GppMaybe, "Safety indicator error", tint = MaterialTheme.colors.SignalDanger)
	}
}
