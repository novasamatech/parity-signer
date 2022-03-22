package io.parity.signer.components.transactionCards

import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import io.parity.signer.ui.theme.Text600
import org.json.JSONObject

@Composable
fun TCFieldName(payload: JSONObject) {
	//TODO: documentation button
	Text(payload.optString("name"), style = MaterialTheme.typography.body2, color = MaterialTheme.colors.Text600)
}
