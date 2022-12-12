package io.parity.signer.screens.scan.transaction.transactionElements

import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import io.parity.signer.ui.theme.SignerTypeface

@Composable
fun TCID(base58: String) {
	Text(
		text = base58,
		style = SignerTypeface.BodyL,
		color = MaterialTheme.colors.primary
	)
}
