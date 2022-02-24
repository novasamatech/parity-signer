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
import org.json.JSONObject

@Composable
fun TCAuthorPublicKey(payload: JSONObject) {
	Row {
		Identicon(identicon = payload.optString("identicon"))
		Column {
			Text("Signed with " + payload.optString("encryption"), style = MaterialTheme.typography.body2, color = MaterialTheme.colors.Text400)
			Text(payload.optString("hex"), style = CryptoTypography.body2, color = MaterialTheme.colors.Crypto400)
		}
	}
}
