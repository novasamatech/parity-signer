package io.parity.signer.screens.keysets.create

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.components.BigButton
import io.parity.signer.components.HeaderBar
import io.parity.signer.models.AlertState
import io.parity.signer.models.Navigator
import io.parity.signer.ui.theme.Bg000
import io.parity.signer.ui.theme.modal
import io.parity.signer.uniffi.Action

@Composable
fun NewSeedMenu(
	alertState: State<AlertState?>,
	navigator: Navigator,
) {
	Column(
		modifier = Modifier.padding(20.dp)
	) {
		HeaderBar(line1 = "ADD SEED", line2 = "Select seed addition method")
		BigButton(
			text = "New seed",
			action = {
				if (alertState.value == AlertState.None)
					navigator.navigate(Action.NEW_SEED)
				else
					navigator.navigate(Action.SHIELD)
			})
		BigButton(
			text = "Recover seed",
			action = {
				if (alertState.value == AlertState.None)
					navigator.navigate(Action.RECOVER_SEED)
				else
					navigator.navigate(Action.SHIELD)
			},
			isShaded = true
		)
	}
}
