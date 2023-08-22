package io.parity.signer.components

import androidx.compose.foundation.layout.*
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Lock
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.components.networkicon.IdentIconImage
import io.parity.signer.domain.BASE58_STYLE_ABBREVIATE
import io.parity.signer.domain.abbreviateString
import io.parity.signer.ui.theme.*
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
			IdentIconImage(identity.address.identicon)
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

