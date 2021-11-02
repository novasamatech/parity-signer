package io.parity.signer.components

import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.AddCircle
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import com.google.android.material.color.MaterialColors
import io.parity.signer.KeyManagerModal
import io.parity.signer.SignerAlert
import io.parity.signer.SignerScreen
import io.parity.signer.models.SignerDataModel

@Composable
fun TopBar(signerDataModel: SignerDataModel) {
	val screen = signerDataModel.signerScreen.observeAsState()
	val keymodal = signerDataModel.keyManagerModal.observeAsState()
	val alert = signerDataModel.alert.observeAsState()

	TopAppBar {
		if (!signerDataModel.isBottom()) {
			IconButton(onClick = {
				signerDataModel.goBack()
			}) {
				Icon(Icons.Default.ArrowBack, contentDescription = "go back")
			}
		}
		Spacer(modifier = Modifier.weight(1f, true))
		Text(signerDataModel.getScreenName())
		Spacer(modifier = Modifier.weight(1f, true))
		if (screen.value == SignerScreen.Log || alert.value != SignerAlert.None) {
			NavbarShield(signerDataModel = signerDataModel)
		} else {
			if (screen.value == SignerScreen.Keys && keymodal.value == KeyManagerModal.SeedSelector) {
				IconButton(onClick = { signerDataModel.newSeedScreenEngage() }) {
					Icon(Icons.Default.AddCircle, "New Seed")
				}
			}
		}
	}
}
