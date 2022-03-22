package io.parity.signer.components

import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.width
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.Text600
import io.parity.signer.ui.theme.Web3Typography

@Composable
fun NetworkLogoName(logo: String, name: String) {
	Text(
		logo,
		style = Web3Typography.h4,
		color = MaterialTheme.colors.Text600
	)
	Spacer(Modifier.width(15.dp))
	Text(
		name,
		style = MaterialTheme.typography.h3,
		color = MaterialTheme.colors.Text600
	)
}
