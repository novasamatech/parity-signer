package io.parity.signer.modals

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.ShieldAlert
import io.parity.signer.alerts.AndroidCalledConfirm
import io.parity.signer.components.BigButton
import io.parity.signer.components.HeaderBar
import io.parity.signer.models.removeSeed
import io.parity.signer.ui.theme.Bg000
import io.parity.signer.ui.theme.modal
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.MSeedMenu

@Composable
fun SeedMenu(
	seedMenu: MSeedMenu,
	shieldAlert: State<ShieldAlert?>,
	button: (Action) -> Unit,
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
							button(Action.BACKUP_SEED)
						else
							button(Action.SHIELD)
					}
				)
				BigButton(
					text = "Derive new key",
					action = {
						if (shieldAlert.value == ShieldAlert.None)
							button(Action.NEW_KEY)
						else
							button(Action.SHIELD)
					},
					isShaded = true,
					isCrypto = true
				)
				BigButton(
					text = "Forget this seed forever",
					isShaded = true,
					isDangerous = true,
					action = {
						confirm = true
					}
				)
			}
		}
	}

	AndroidCalledConfirm(
		show = confirm,
		header = "Forget this seed forever?",
		text = "This seed will be removed for all networks. " +
			"This is not reversible. Are you sure?",
		back = { confirm = false },
		forward = {
			seedMenu.seed.let {
				if (seedMenu.seed.isNotBlank()) removeSeed(it)
			}
		},
		backText = "Cancel",
		forwardText = "Remove seed"
	)
}
