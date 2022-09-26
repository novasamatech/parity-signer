package io.parity.signer.components2

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
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
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.IdentIcon
import io.parity.signer.models.abbreviateString
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.*
import io.parity.signer.uniffi.Address


@Composable
fun KeyCard(model: KeyCardModel) {
	Row(
		Modifier
			.fillMaxWidth()
			.padding(16.dp)
	) {

		//left
		Column() {
			Row(
				verticalAlignment = Alignment.CenterVertically
			) {
				Text(
					model.path,
					color = MaterialTheme.colors.textSecondary,
					style = TypefaceNew.CaptionM,
				)
				if (model.hasPwd) {
					Text(
						" •••• ",
						color = MaterialTheme.colors.textSecondary,
						style = TypefaceNew.CaptionM,
					)
					Icon(
						Icons.Default.Lock,
						contentDescription = stringResource(R.string.description_locked_icon),
						tint = MaterialTheme.colors.textSecondary,
					)
				}
			}

			Spacer(Modifier.padding(top = 4.dp))

			Text(
				model.seedName,
				color = MaterialTheme.colors.primary,
				style = TypefaceNew.LabelS,
			)

			Spacer(Modifier.padding(top = 10.dp))

			showBase58Collapsible(model)
		}

		Spacer(Modifier.weight(1f))

		//right()
		Column(horizontalAlignment = Alignment.End) {
			Box(contentAlignment = Alignment.TopEnd) {
				IdentIcon(model.identIcon, 36.dp)
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

			Spacer(Modifier.padding(top = 14.dp))

			Box(
				modifier = Modifier
					.background(
						MaterialTheme.colors.fill12,
						RoundedCornerShape(12.dp)
					)
					.padding(horizontal = 8.dp, vertical = 2.dp),
				contentAlignment = Alignment.Center,
			) {
				Text(
					model.network,
					color = MaterialTheme.colors.textTertiary,
					style = TypefaceNew.CaptionM,
				)
			}
		}
	}
}

@Composable
private fun showBase58Collapsible(model: KeyCardModel) {
	val expanded = remember { mutableStateOf(false) }
	Row(
		verticalAlignment = Alignment.CenterVertically,
		modifier = Modifier.clickable { expanded.value = !expanded.value }
	) {
		if (expanded.value) {
			Text(
				model.base58,
				color = MaterialTheme.colors.textTertiary,
				style = TypefaceNew.BodyM
			)
		} else {
			Text(
				model.base58.abbreviateString(8),
				color = MaterialTheme.colors.textTertiary,
				style = TypefaceNew.BodyM,
				maxLines = 1,
			)
			Spacer(modifier = Modifier.padding(horizontal = 4.dp))
			Icon(
				imageVector = Icons.Default.KeyboardArrowDown,
				modifier = Modifier.size(20.dp),
				contentDescription = stringResource(R.string.description_expand_button),
				tint = MaterialTheme.colors.textSecondary
			)
		}
	}
}

data class KeyCardModel(
	val network: String,
	val base58: String,
	val path: String,
	val identIcon: List<UByte>,
	val seedName: String,
	val hasPwd: Boolean = false,
	val multiselect: Boolean? = null,
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
			network = "Polkadot",
			base58 = "5F3sa2TJAWMqDhXG6jhV4N8ko9SxwGy8TpaNS1repo5EYjQX",
			path = "//polkadot//path",
			identIcon = PreviewData.exampleIdenticon,
			seedName = "Seed Name",
			hasPwd = false,
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
	backgroundColor = 0xFFFFFFFF
)
@Composable
private fun PreviewKeyCard() {
	SignerNewTheme {
		KeyCard(model = KeyCardModel.createStub())
	}
}
