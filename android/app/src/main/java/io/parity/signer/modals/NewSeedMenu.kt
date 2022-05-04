package io.parity.signer.modals

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.components.BigButton
import io.parity.signer.components.HeaderBar
import io.parity.signer.ui.theme.Bg000
import io.parity.signer.ui.theme.modal
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.ShieldAlert

@Composable
fun NewSeedMenu(
	shieldAlert: ShieldAlert?,
	button: (Action) -> Unit
) {
	Column {
		Spacer(Modifier.weight(1f))
		Surface(
			color = MaterialTheme.colors.Bg000,
			shape = MaterialTheme.shapes.modal
		) {
			Column(
				modifier = Modifier.padding(20.dp)
			) {
				HeaderBar(line1 = "ADD SEED", line2 = "Select seed addition method")
				BigButton(
					text = "New seed",
					action = {
						if (shieldAlert == null)
							button(Action.NEW_SEED)
						else
							button(Action.SHIELD)
					})
				BigButton(
					text = "Recover seed",
					action = {
						if (shieldAlert == null)
							button(Action.RECOVER_SEED)
						else
							button(Action.SHIELD)
					},
					isShaded = true
				)
			}
		}
	}
}
