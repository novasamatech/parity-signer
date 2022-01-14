package io.parity.signer.components

import androidx.compose.material.Icon
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import io.parity.signer.ShieldAlert
import io.parity.signer.models.SignerDataModel

@Composable
fun NavbarShield(signerDataModel: SignerDataModel) {
	val alert = signerDataModel.alertState.observeAsState()

	when(alert.value) {
		ShieldAlert.None -> Icon(Icons.Default.GppGood, "device is safe")
		ShieldAlert.Active -> Icon(Icons.Default.GppBad, "device is online")
		ShieldAlert.Past -> Icon(Icons.Default.GppBad, "potential security breach")
		null -> Icon(Icons.Default.GppMaybe, "Safety indicator error")
	}
}
