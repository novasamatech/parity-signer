package io.parity.signer.components.transactionCards

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import io.parity.signer.components.Identicon
import io.parity.signer.ui.theme.Crypto400
import io.parity.signer.ui.theme.CryptoTypography
import io.parity.signer.ui.theme.Text400
import io.parity.signer.uniffi.MVerifierDetails

@Composable
fun TCAuthorPublicKey(key: MVerifierDetails) {
	Row {
		Identicon(identicon = key.identicon)
		Column {
			Text(
				"Signed with " + key.encryption,
				style = MaterialTheme.typography.body2,
				color = MaterialTheme.colors.Text400
			)
			Text(
				key.publicKey,
				style = CryptoTypography.body2,
				color = MaterialTheme.colors.Crypto400
			)
		}
	}
}
