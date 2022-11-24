package io.parity.signer.components

import androidx.compose.foundation.layout.*
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.CheckCircle
import androidx.compose.material.icons.filled.Lock
import androidx.compose.material.icons.outlined.Circle
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.models.BASE58_STYLE_ABBREVIATE
import io.parity.signer.models.abbreviateString
import io.parity.signer.ui.theme.*
import io.parity.signer.uniffi.Address
import io.parity.signer.uniffi.MAddressCard

/**
 * A card to show key info; only visual things.
 */
@Deprecated("Use KeyCard for new screens")
@Composable
fun KeyCardOld(identity: MAddressCard, multiselectMode: Boolean = false) {
	Row(
		verticalAlignment = Alignment.CenterVertically,
		modifier = Modifier
			.padding(8.dp)
	) {
		Box(contentAlignment = Alignment.BottomEnd) {
			IdentIcon(identity.address.identicon)
			if (multiselectMode) {
				identity.multiselect?.let {
					if (it) {
						Icon(
							Icons.Default.CheckCircle,
							"Not multiselected",
							tint = MaterialTheme.colors.Action400
						)
					} else {
						Icon(
							Icons.Outlined.Circle,
							"Multiselected",
							tint = MaterialTheme.colors.Action400
						)
					}
				}
			}
		}
		Spacer(modifier = Modifier.width(10.dp))
		Column {
			Row(
				verticalAlignment = Alignment.CenterVertically
			) {
				Text(
					identity.address.seedName,
					color = MaterialTheme.colors.Text600,
					style = MaterialTheme.typography.subtitle1
				)
				Text(
					identity.address.path,
					color = MaterialTheme.colors.Crypto400,
					style = CryptoTypography.body2
				)
				if (identity.address.hasPwd) {
					Text(
						"///",
						color = MaterialTheme.colors.Crypto400,
						style = CryptoTypography.body2
					)
					Icon(
						Icons.Default.Lock,
						contentDescription = "Locked account",
						tint = MaterialTheme.colors.Crypto400
					)
				}
			}
			Text(
				identity.base58.abbreviateString(BASE58_STYLE_ABBREVIATE),
				color = MaterialTheme.colors.Text400,
				style = CryptoTypography.body2
			)
		}
	}
}

