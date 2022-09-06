package io.parity.signer.components.transactionCards

import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import io.parity.signer.ui.theme.Text600
import io.parity.signer.uniffi.MscFieldNumber

@Composable
fun TCFieldNumber(fieldNumber: MscFieldNumber) {
	//TODO: documentation button
	Text(
		fieldNumber.number,
		style = MaterialTheme.typography.body2,
		color = MaterialTheme.colors.Text600
	)
}
