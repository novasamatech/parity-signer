package io.parity.signer.components2

import android.content.res.Configuration
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.CheckCircle
import androidx.compose.material.icons.filled.KeyboardArrowDown
import androidx.compose.material.icons.filled.Lock
import androidx.compose.material.icons.outlined.Circle
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.components.Identicon
import io.parity.signer.models.abbreviateString
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.*
import io.parity.signer.uniffi.Address
import io.parity.signer.uniffi.MKeyDetails


@Composable
fun KeyCard(model: KeyCardModel) {
	val expanded = remember { mutableStateOf(false) }
	Row(
		verticalAlignment = Alignment.CenterVertically,
		modifier = Modifier
			.padding(8.dp)
	) {

		Box(contentAlignment = Alignment.BottomEnd) {
			Identicon(model.identIcon)
			model.multiselect?.let {
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
		Spacer(modifier = Modifier.width(10.dp))
		Column {
			Text(
				model.network,
				color = MaterialTheme.colors.textSecondary,
				style = MaterialTheme.typography.subtitle2
			)
			Row(
				verticalAlignment = Alignment.CenterVertically
			) {
				Text(
					model.seedName,
					color = MaterialTheme.colors.primary,
					style = MaterialTheme.typography.subtitle1
				)
				Text(
					model.path,
					color = MaterialTheme.colors.primary,
					style = CryptoTypography.body2
				)
				if (model.hasPwd) {
					Text(
						"///",
						color = MaterialTheme.colors.primary,
						style = CryptoTypography.body2
					)
					Icon(
						Icons.Default.Lock,
						contentDescription = "Locked account",
						tint = MaterialTheme.colors.primary,
					)
				}
			}
			Row(
				verticalAlignment = Alignment.CenterVertically,
				modifier = Modifier.clickable { expanded.value = !expanded.value }
			) {
				if (expanded.value) {
					Text(
						model.base58,
						color = MaterialTheme.colors.textTertiary,
						style = CryptoTypography.body2
					)
				} else {
					Text(
						model.base58.abbreviateString(8),
						color = MaterialTheme.colors.textTertiary,
						style = CryptoTypography.body2,
						maxLines = 1,
					)
					Icon(
						imageVector = Icons.Default.KeyboardArrowDown,
						modifier = Modifier
							.padding(start = 10.dp)
							.size(24.dp),
						contentDescription = "expand icon",
						tint = MaterialTheme.colors.textSecondary
					)
				}
			}
		}
	}
}

data class KeyCardModel(
	val network: String,
	val base58: String,
	val path: String,
	val hasPwd: Boolean,
	val identIcon: List<UByte>,
	val seedName: String,
	val multiselect: Boolean?,
) {
	companion object {
		/**
		 * @param networkTitle probably from keyDetails.networkInfo.networkTitle
		 */
		fun fromAddress(address: Address, networkTitle: String): KeyCardModel =
			KeyCardModel(
				network = networkTitle,
				base58 = address.base58,
				path = address.path,
				hasPwd = address.hasPwd,
				identIcon = address.identicon,
				seedName = address.seedName,
				multiselect = address.multiselect,
			)

		fun createStub() = KeyCardModel(
			network = "polkadot",
			base58 = "kg;dlfgopdifopbcvjblkcvjpiobjvlkjvlkbjnlkfd",
			path = "path long path",
			hasPwd = false,
			identIcon = PreviewData.exampleIdenticon,
			seedName = "seed name",
			multiselect = null,
		)
	}
}

@Preview(
	name = "day",
	group = "themes",
	uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true,
)
@Preview(
	name = "dark theme",
	group = "themes",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	backgroundColor = 0xFF000000,
	showBackground = true,
)
@Composable
private fun PreviewKeyCard() {
	SignerNewTheme() {
		KeyCard(model = KeyCardModel.createStub())
	}
}
