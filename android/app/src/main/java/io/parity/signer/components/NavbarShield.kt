package io.parity.signer.components

import androidx.compose.material.Icon
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Check
import androidx.compose.material.icons.filled.Delete
import androidx.compose.material.icons.filled.Warning
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import io.parity.signer.SignerAlert
import io.parity.signer.models.SignerDataModel

@Composable
fun NavbarShield(signerDataModel: SignerDataModel) {
	val alert = signerDataModel.alert.observeAsState()

	when(alert.value) {
		SignerAlert.None -> Icon(Icons.Default.Check, "device is safe")
		SignerAlert.Active -> Icon(Icons.Default.Warning, "device is online")
		SignerAlert.Past -> Icon(Icons.Default.Warning, "potential security breach")
		null -> Icon(Icons.Default.Delete, "Safety indicator error")
	}
}
