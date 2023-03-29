package io.parity.signer.screens.settings

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.material.Text
import androidx.compose.runtime.*
import io.parity.signer.alerts.AndroidCalledConfirm
import io.parity.signer.components.BigButton
import io.parity.signer.components.IdentIcon
import io.parity.signer.components.toImageContent
import io.parity.signer.uniffi.MVerifierDetails

@Composable
//todo dmitry remove it
fun VerifierScreenOld(
	verifierDetails: MVerifierDetails,
	wipeToJailbreak: () -> Unit
) {
	var jailbreakAlert by remember { mutableStateOf(false) }

	Column {
		Row {
			IdentIcon(identicon = verifierDetails.identicon.toImageContent())
			Column {
				Text("General verifier certificate")
				Text(verifierDetails.publicKey)
				Text("encryption: " + verifierDetails.encryption)
			}
		}
		BigButton(
			text = "Remove general certificate",
			action = { jailbreakAlert = true },
			isDangerous = true,
			isShaded = true
		)
	}

	AndroidCalledConfirm(
		show = jailbreakAlert,
		header = "Wipe ALL data?",
		text = "Remove all data and set general verifier blank so that it could be set later. This operation can not be reverted. Do not proceed unless you absolutely know what you are doing, there is no need to use this procedure in most cases. Misusing this feature may lead to loss of funds!",
		back = { jailbreakAlert = false },
		forward = { wipeToJailbreak() },
		backText = "Cancel",
		forwardText = "I understand"
	)
}
