package io.parity.signer.modals

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import io.parity.signer.ButtonID
import io.parity.signer.components.BigButton
import io.parity.signer.components.HeaderBar
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton

@Composable
fun NewSeedMenu(signerDataModel: SignerDataModel) {
	Column () {
		Spacer(Modifier.weight(1f))
		HeaderBar(line1 = "ADD SEED", line2 = "Select seed addition method")
		BigButton(text = "New seed", action = { signerDataModel.pushButton(ButtonID.NewSeed) })
		BigButton(text = "Recover seed", action = { signerDataModel.pushButton(ButtonID.RecoverSeed) })
	}
}
