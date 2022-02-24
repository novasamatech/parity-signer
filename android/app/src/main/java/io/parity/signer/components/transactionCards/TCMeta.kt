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
import io.parity.signer.ui.theme.Text600
import org.json.JSONObject

@Composable
fun TCMeta(payload: JSONObject) {
	Row {
		Identicon(identicon = payload.optString("meta_id_pic"))
		Column {
			Text("Add metadata", style = MaterialTheme.typography.body2, color = MaterialTheme.colors.Text600)
			Text(payload.optString("specname") + " " + payload.optString("spec_version"), style = CryptoTypography.body2, color = MaterialTheme.colors.Crypto400)
			Text(payload.optString("meta_hash"), style = CryptoTypography.body2, color = MaterialTheme.colors.Text400)
		}
	}
}
