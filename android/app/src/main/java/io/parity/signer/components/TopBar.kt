package io.parity.signer.components

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.width
import androidx.compose.material.*
import androidx.compose.material.ButtonDefaults.buttonColors
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.AddCircleOutline
import androidx.compose.material.icons.filled.ChevronLeft
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.scale
import androidx.compose.ui.unit.dp
import io.parity.signer.ButtonID
import io.parity.signer.models.*
import io.parity.signer.ui.theme.Action400
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
			modifier = Modifier
				.weight(0.2f, fill = true)
				.width(72.dp)
		) {
			if (backButton.value == true) {
				Button(
					colors = buttonColors(
						contentColor = MaterialTheme.colors.Action400,
						backgroundColor = MaterialTheme.colors.Bg100
					),
					onClick = {
						signerDataModel.pushButton(ButtonID.GoBack)
					}) {
					Icon(
						Icons.Default.ChevronLeft,
						"go back",
						tint = MaterialTheme.colors.Action400,
						modifier = Modifier.scale(2f)
					)
				}
			}
		}
		Row(
			horizontalArrangement = Arrangement.Center,
			modifier = Modifier.weight(0.6f, fill = true)
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
			modifier = Modifier
				.weight(0.2f, fill = true)
				.width(72.dp)
		) {
			IconButton(onClick = { signerDataModel.pushButton(ButtonID.RightButton) }) {
				when (rightButton.value) {
					"NewSeed" -> {
						Icon(
							Icons.Default.AddCircleOutline,
							"New Seed",
							tint = MaterialTheme.colors.Action400
						)
					}
					"Backup" -> {
						Icon(
							Icons.Default.MoreVert,
							"Seed backup",
							tint = MaterialTheme.colors.Action400
						)
					}
					"LogRight" -> {
						Icon(
							Icons.Default.MoreVert,
							"Log menu",
							tint = MaterialTheme.colors.Action400
						)
					}
					"MultiSelect" -> {

					}
					"None" -> {

					}
					else -> {
						Icon(
							Icons.Default.MoreVert,
							"Menu",
							tint = MaterialTheme.colors.Action400
						)
					}
				}
			}
			IconButton(onClick = { signerDataModel.pushButton(ButtonID.Shield) }) {
				NavbarShield(signerDataModel = signerDataModel)
			}
		}
	}
}
