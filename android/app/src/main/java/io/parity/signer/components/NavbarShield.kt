package io.parity.signer.components

import androidx.compose.material.Icon
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import io.parity.signer.SignerAlert
import io.parity.signer.models.SignerDataModel

@Composable
fun NavbarShield(signerDataModel: SignerDataModel) {
	val alert = signerDataModel.alert.observeAsState()

	when(alert.value) {
		SignerAlert.None -> Icon(Icons.Default.GppGood, "device is safe")
		SignerAlert.Active -> Icon(Icons.Default.GppBad, "device is online")
		SignerAlert.Past -> Icon(Icons.Default.GppBad, "potential security breach")
		null -> Icon(Icons.Default.GppMaybe, "Safety indicator error")
	}
}
