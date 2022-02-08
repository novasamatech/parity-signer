package io.parity.signer.components

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.width
import androidx.compose.material.*
import androidx.compose.material.ButtonDefaults.buttonColors
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.AddCircleOutline
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
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
		backgroundColor = MaterialTheme.colors.Bg100
	) {
		Row(
			horizontalArrangement = Arrangement.Start,
			modifier = Modifier.weight(0.3f, fill = true).width(72.dp)
		) {
			if (backButton.value == true) {
				Button(
					colors = buttonColors(
						contentColor = MaterialTheme.colors.Text500,
						backgroundColor = MaterialTheme.colors.Bg100
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
				style = if (screenNameType.value == "h1") {
					MaterialTheme.typography.h2
				} else {
					MaterialTheme.typography.h4
				}
			)
		}
		Row(
			horizontalArrangement = Arrangement.End,
			modifier = Modifier.weight(0.3f, fill = true).width(72.dp)
		) {
			IconButton(onClick = { signerDataModel.pushButton(ButtonID.RightButton) }) {
				when (rightButton.value) {
					"NewSeed" -> {
						Icon(Icons.Default.AddCircleOutline, "New Seed")
					}
					"Backup" -> {
						Icon(Icons.Default.MoreVert, "Seed backup")
					}
					"LogRight" -> {
						Icon(Icons.Default.MoreVert, "Seed backup")
					}
					"MultiSelect" -> {

					}
					"None" -> {

					}
					else -> {
						Icon(Icons.Default.MoreVert, "Seed backup")
					}
				}
			}
			IconButton(onClick = { signerDataModel.pushButton(ButtonID.Shield) }) {
				NavbarShield(signerDataModel = signerDataModel)
			}
		}
	}
}
