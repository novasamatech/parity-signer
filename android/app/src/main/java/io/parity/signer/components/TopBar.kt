package io.parity.signer.components

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.material.*
import androidx.compose.material.ButtonDefaults.buttonColors
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.AddCircle
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import io.parity.signer.ButtonID
import io.parity.signer.SignerScreen
import io.parity.signer.models.*
import io.parity.signer.ui.theme.Bg100
import io.parity.signer.ui.theme.Text500

@Composable
fun TopBar(signerDataModel: SignerDataModel) {
	val screen = signerDataModel.signerScreen.observeAsState()
	val screenName = signerDataModel.screenName.observeAsState()
	val backButton = signerDataModel.backButton.observeAsState()
	val keymodal = signerDataModel.signerModal.observeAsState()
	val alert = signerDataModel.alert.observeAsState()

	TopAppBar(
		backgroundColor = Bg100
	) {
		Row(
			horizontalArrangement = Arrangement.Start,
			modifier = Modifier.weight(0.3f, fill = true)) {
			if (backButton.value == true) {
				Button(
					colors = buttonColors(
						contentColor = Text500,
						backgroundColor = Bg100
					),
					onClick = {
					signerDataModel.pushButton(ButtonID.GoBack)
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
			signerDataModel.screenName.value?:""
		)
		}
		Row(
			horizontalArrangement = Arrangement.End,
			modifier = Modifier.weight(0.3f, fill = true)
		) {
			if (screen.value == SignerScreen.SeedSelector) {
				IconButton(onClick = { TODO() }) {
					Icon(Icons.Default.AddCircle, "New Seed")
				}
			}
			NavbarShield(signerDataModel = signerDataModel)
		}
	}
}
