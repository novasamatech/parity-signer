package io.parity.signer.components

import androidx.compose.foundation.gestures.draggable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material.*
import androidx.compose.material.ButtonDefaults.buttonColors
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.AddCircle
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import com.google.android.material.color.MaterialColors
import io.parity.signer.KeyManagerModal
import io.parity.signer.SignerAlert
import io.parity.signer.SignerScreen
import io.parity.signer.models.*
import io.parity.signer.ui.theme.Bg100
import io.parity.signer.ui.theme.Text500

@Composable
fun TopBar(signerDataModel: SignerDataModel) {
	val screen = signerDataModel.signerScreen.observeAsState()
	val keymodal = signerDataModel.keyManagerModal.observeAsState()
	val alert = signerDataModel.alert.observeAsState()

	TopAppBar(
		backgroundColor = Bg100
	) {
		Row(
			horizontalArrangement = Arrangement.Start,
			modifier = Modifier.weight(0.3f, fill = true)) {
			if (!signerDataModel.isBottom()) {
				Button(
					colors = buttonColors(
						contentColor = Text500,
						backgroundColor = Bg100
					),
					onClick = {
					signerDataModel.goBack()
				}) {
					Text("Back")
				}
			}
		}
		Row(
			horizontalArrangement = Arrangement.Center,
			modifier = Modifier.weight(0.4f, fill = true)
		){
		Text(
			signerDataModel.getScreenName()
		)
		}
		Row(
			horizontalArrangement = Arrangement.End,
			modifier = Modifier.weight(0.3f, fill = true)
		) {
			if (screen.value == SignerScreen.Keys && keymodal.value == KeyManagerModal.SeedSelector) {
				IconButton(onClick = { signerDataModel.newSeedScreenEngage() }) {
					Icon(Icons.Default.AddCircle, "New Seed")
				}
			}
			NavbarShield(signerDataModel = signerDataModel)
		}
	}
}
