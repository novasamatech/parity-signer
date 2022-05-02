package io.parity.signer.components.transactionCards

import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import io.parity.signer.ui.theme.Text600
import io.parity.signer.uniffi.MscFieldName
import org.json.JSONObject

@Composable
fun TCFieldName(fieldName: MscFieldName) {
	//TODO: documentation button
	Text(
		fieldName.name,
		style = MaterialTheme.typography.body2,
		color = MaterialTheme.colors.Text600
	)
}
