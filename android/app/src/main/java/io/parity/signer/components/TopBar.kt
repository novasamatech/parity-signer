package io.parity.signer.components

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.material.*
import androidx.compose.material.ButtonDefaults.buttonColors
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.AddCircleOutline
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import io.parity.signer.ButtonID
import io.parity.signer.models.*
import io.parity.signer.ui.theme.Bg100
import io.parity.signer.ui.theme.Text500

@Composable
fun TopBar(signerDataModel: SignerDataModel) {
	val backButton = signerDataModel.back.observeAsState()
	val screenName = signerDataModel.screenLabel.observeAsState()
	val screenNameType = signerDataModel.screenNameType.observeAsState()
	val rightButton = signerDataModel.rightButton.observeAsState()

	TopAppBar(
		backgroundColor = Bg100
	) {
		Row(
			horizontalArrangement = Arrangement.Start,
			modifier = Modifier.weight(0.3f, fill = true)
		) {
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
		) {
			Text(
				screenName.value ?: "",
				style = if (screenNameType.value == "h4") {
					MaterialTheme.typography.h4
				} else {
					MaterialTheme.typography.h1
				}
			)
		}
		Row(
			horizontalArrangement = Arrangement.End,
			modifier = Modifier.weight(0.3f, fill = true)
		) {
			IconButton(onClick = { signerDataModel.pushButton(ButtonID.RightButton) }) {
				when (rightButton.value) {
					"NewSeed" -> {
						Icon(Icons.Default.AddCircleOutline, "New Seed")
					}
					"Backup" -> {
						Icon(Icons.Default.MoreVert, "Seed backup")
					}
					else -> {
					}
				}
			}
			IconButton(onClick = { signerDataModel.pushButton(ButtonID.Shield) }) {
				NavbarShield(signerDataModel = signerDataModel)
			}
		}
	}
}
