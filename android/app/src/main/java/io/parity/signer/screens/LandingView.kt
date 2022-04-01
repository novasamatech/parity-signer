package io.parity.signer.screens

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.selection.toggleable
import androidx.compose.material.Checkbox
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.unit.dp
import com.halilibo.richtext.ui.material.SetupMaterialRichText
import io.parity.signer.alerts.AndroidCalledConfirm
import io.parity.signer.components.BigButton
import io.parity.signer.components.Documents
import io.parity.signer.models.SignerDataModel
import io.parity.signer.ui.theme.Action400
import io.parity.signer.ui.theme.Bg100

/**
 * First screen with legal consent request
 */
@Composable
fun LandingView(
	onBoard: () -> Unit
) {
	var confirm by remember { mutableStateOf(false) }
	var tacAccept by remember { mutableStateOf(false) }
	var ppAccept by remember { mutableStateOf(false) }

	Box {
			Documents()
		Column(Modifier.padding(horizontal = 20.dp)) {
			Spacer(Modifier.weight(1f))
			Surface(color = MaterialTheme.colors.Bg100) {
				Column {
					Spacer(Modifier.height(16.dp))
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
						Spacer(Modifier.width(8.dp))
						Text("I agree to the terms and conditions", color = MaterialTheme.colors.Action400)
						Spacer(Modifier.weight(1f))
					}
					Spacer(Modifier.height(16.dp))
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
						Spacer(Modifier.width(8.dp))
						Text("I agree to the privacy policy", color = MaterialTheme.colors.Action400)
						Spacer(Modifier.weight(1f))
					}
					Spacer(Modifier.height(16.dp))
					BigButton(
						text = "Next", action = { confirm = true },
						isDisabled = !(tacAccept && ppAccept)
					)
					Spacer(Modifier.height(16.dp))
				}
			}
		}
	}

	AndroidCalledConfirm(
		show = confirm,
		header = "Accept terms and conditions and privacy policy?",
		back = { confirm = false },
		forward = { onBoard() },
		backText = "Decline",
		forwardText = "Accept"
	)
}
