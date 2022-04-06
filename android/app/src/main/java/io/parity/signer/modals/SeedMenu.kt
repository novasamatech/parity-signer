package io.parity.signer.modals

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.ButtonID
import io.parity.signer.ShieldAlert
import io.parity.signer.alerts.AndroidCalledConfirm
import io.parity.signer.components.BigButton
import io.parity.signer.components.HeaderBar
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton
import io.parity.signer.models.removeSeed
import io.parity.signer.ui.theme.Bg000
import io.parity.signer.ui.theme.modal
import org.json.JSONObject

@Composable
fun SeedMenu(
	modalData: State<JSONObject?>,
	shieldAlert: State<ShieldAlert?>,
	button: (ButtonID) -> Unit,
	removeSeed: (String) -> Unit
) {
	var confirm by remember { mutableStateOf(false) }

	Column {
		Spacer(Modifier.weight(1f))
		Surface(
			color = MaterialTheme.colors.Bg000,
			shape = MaterialTheme.shapes.modal
		) {
			Column(
				modifier = Modifier.padding(20.dp)
			) {
				HeaderBar(line1 = "SEED MENU", line2 = "Select action")
				BigButton(
					text = "Backup",
					action = {
						if (shieldAlert.value == ShieldAlert.None)
							button(ButtonID.BackupSeed)
						else
							button(ButtonID.Shield)
					})
				BigButton(
					text = "Derive new key",
					action = {
						if (shieldAlert.value == ShieldAlert.None)
							button(ButtonID.NewKey)
						else
							button(ButtonID.Shield)
					},
					isShaded = true,
					isCrypto = true
				)
				BigButton(
					text = "Forget this seed forever",
					isShaded = true,
					isDangerous = true,
					action = {
						val seedName =
							modalData.value?.optString("seed") ?: ""
						removeSeed(seedName)
					}
				)
			}
		}
	}

	AndroidCalledConfirm(
		show = confirm,
		header = "Forget this seed forever?",
		text = "This seed will be removed for all networks. This is not reversible. Are you sure?",
		back = { confirm = false },
		forward = {
			modalData.value?.optString("seed")?.let {
				if (it.isNotBlank()) removeSeed(it)
			}
		},
		backText = "Cancel",
		forwardText = "Remove seed"
	)
}
