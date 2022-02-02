package io.parity.signer.screens

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.selection.toggleable
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.Role
import io.parity.signer.components.BigButton
import io.parity.signer.components.Documents
import io.parity.signer.models.SignerDataModel
import io.parity.signer.ui.theme.Action400
import io.parity.signer.ui.theme.Action600

@Composable
fun LandingView(signerDataModel: SignerDataModel) {
	var confirm by remember { mutableStateOf(false) }
	var tacAccept by remember { mutableStateOf(false) }
	var ppAccept by remember { mutableStateOf(false) }

	Column {
		Documents()
		Row(
			verticalAlignment = Alignment.CenterVertically,
			modifier = Modifier.toggleable(
				value = tacAccept,
				role = Role.Checkbox,
				onValueChange = { tacAccept = it }
			)) {
			Checkbox(
				checked = tacAccept,
				onCheckedChange = null
			)
			Text("I agree to the terms and conditions", color = Action400)
			Spacer(Modifier.weight(1f))
		}
		Row(
			verticalAlignment = Alignment.CenterVertically,
			modifier = Modifier.toggleable(
				value = ppAccept,
				role = Role.Checkbox,
				onValueChange = { ppAccept = it }
			)) {
			Checkbox(
				checked = ppAccept,
				onCheckedChange = null
			)
			Text("I agree to the privacy policy", color = Action400)
			Spacer(Modifier.weight(1f))
		}
		BigButton(
			text = "Next", action = { confirm = true },
			isDisabled = !(tacAccept && ppAccept)
		)
		if (confirm) {
			AlertDialog(
				onDismissRequest = { confirm = false },
				buttons = {
					Button(
						colors = ButtonDefaults.buttonColors(
							backgroundColor = MaterialTheme.colors.background,
							contentColor = MaterialTheme.colors.onBackground,
						),
						onClick = {
							confirm = false
						}
					) {
						Text("Decline")
					}
					Button(
						colors = ButtonDefaults.buttonColors(
							backgroundColor = MaterialTheme.colors.background,
							contentColor = MaterialTheme.colors.onBackground,
						),
						onClick = {
							signerDataModel.onBoard()
						}
					) {
						Text("Accept")
					}
				},
				title = { Text("Accept terms and conditions and privacy policy?") },
				text = { }
			)
		}
	}
}
