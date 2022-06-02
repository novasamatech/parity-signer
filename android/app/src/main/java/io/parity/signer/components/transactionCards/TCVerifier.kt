package io.parity.signer.components.transactionCards

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import io.parity.signer.components.Identicon
import io.parity.signer.ui.theme.Crypto400
import io.parity.signer.uniffi.MVerifierDetails

@Composable
fun TCVerifier(verifier: MVerifierDetails) {
	Column {
		Text("VERIFIER CERTIFICATE")
		Row {
			Identicon(identicon = verifier.identicon)
			Column {
				Row {
					Text("key:")
					Text(verifier.publicKey, color = MaterialTheme.colors.Crypto400)
				}
				Row {
					Text("crypto:")
					Text(verifier.encryption, color = MaterialTheme.colors.Crypto400)
				}
			}
		}
	}
}
