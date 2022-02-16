package io.parity.signer.components

import androidx.compose.foundation.layout.*
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.models.abbreviateString
import io.parity.signer.ui.theme.Bg200
import io.parity.signer.ui.theme.CryptoTypography
import io.parity.signer.ui.theme.Text400
import io.parity.signer.ui.theme.Text600

@Composable
fun SeedCard(
	seedName: String,
	identicon: String,
	base58: String = "",
	showAddress: Boolean = false
) {
	Surface(
		shape = MaterialTheme.shapes.medium,
		color = MaterialTheme.colors.Bg200,
		modifier = Modifier.heightIn(47.dp).padding(8.dp)
	) {
		Row(
			verticalAlignment = Alignment.CenterVertically,
		) {
			Identicon(identicon)
			Spacer(modifier = Modifier.width(10.dp))
			Column {
				Text(
					seedName,
					color = MaterialTheme.colors.Text600,
					style = MaterialTheme.typography.subtitle1
				)
				if (showAddress) {
					Text(
						base58.abbreviateString(8),
						color = MaterialTheme.colors.Text400,
						style = CryptoTypography.body1
					)
				}
			}
		}
	}
}
