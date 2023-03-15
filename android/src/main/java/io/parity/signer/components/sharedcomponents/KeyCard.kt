package io.parity.signer.components.sharedcomponents

import android.content.res.Configuration
import androidx.compose.animation.animateContentSize
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
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.IdentIconWithNetwork
import io.parity.signer.components.ImageContent
import io.parity.signer.components.toImageContent
import io.parity.signer.domain.BASE58_STYLE_ABBREVIATE
import io.parity.signer.domain.KeyModel
import io.parity.signer.domain.abbreviateString
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.*
import io.parity.signer.uniffi.Address
import io.parity.signer.uniffi.MAddressCard
import java.util.*


@Composable
fun KeyCard(model: KeyCardModel) {
	Row(
		Modifier
			.fillMaxWidth()
			.padding(16.dp)
	) {

		//left
		Column(Modifier.weight(1f)) {
			Row(
				verticalAlignment = Alignment.CenterVertically
			) {
				Text(
					model.cardBase.path,
					color = MaterialTheme.colors.textSecondary,
					style = SignerTypeface.CaptionM,
				)
				if (model.cardBase.hasPassword) {
					Text(
						" •••• ",
						color = MaterialTheme.colors.textSecondary,
						style = SignerTypeface.CaptionM,
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
				model.cardBase.seedName,
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.LabelS,
			)

			Spacer(Modifier.padding(top = 10.dp))

			Box(modifier = Modifier.padding(end = 24.dp)) {
				ShowBase58Collapsible(model.cardBase.base58)
			}
		}


		//right()
		Column(horizontalAlignment = Alignment.End) {
			Box(contentAlignment = Alignment.TopEnd) {
				IdentIconWithNetwork(model.cardBase.identIcon, model.network, 36.dp)
				model.cardBase.multiselect?.let {
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

			val networkName = model.network
			NetworkLabel(networkName)
		}
	}
}

@Composable
fun NetworkLabel(networkName: String, modifier: Modifier = Modifier) {
	Text(
		networkName,
		color = MaterialTheme.colors.textTertiary,
		style = SignerTypeface.CaptionM,
		modifier = modifier
			.background(
				MaterialTheme.colors.fill12,
				RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius))
			)
			.padding(horizontal = 8.dp, vertical = 2.dp)
	)
}

@Composable
fun KeySeedCard(seedTitle: String, base58: String) {
	Column(
		Modifier
			.fillMaxWidth()
			.padding(16.dp)
	) {
		Text(
			seedTitle,
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.LabelS,
		)
		ShowBase58Collapsible(base58)
	}
}

@Composable
fun ShowBase58Collapsible(base58: String) {
	val expanded = remember { mutableStateOf(false) }
	Row(
		verticalAlignment = Alignment.CenterVertically,
		modifier = Modifier
			.clickable { expanded.value = !expanded.value }
			.animateContentSize()
	) {
		if (expanded.value) {
			Text(
				base58,
				color = MaterialTheme.colors.textTertiary,
				style = SignerTypeface.BodyM
			)
		} else {
			Text(
				base58.abbreviateString(BASE58_STYLE_ABBREVIATE),
				color = MaterialTheme.colors.textTertiary,
				style = SignerTypeface.BodyM,
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
	val cardBase: KeyCardModelBase,
) {
	companion object {

		fun fromKeyModel(model: KeyModel, networkTitle: String): KeyCardModel =
			KeyCardModel(
				network = networkTitle.replaceFirstChar {
					if (it.isLowerCase()) it.titlecase() else it.toString()
				},
				cardBase = KeyCardModelBase.fromKeyModel(model)
			)

		/**
		 * @param networkTitle probably from keyDetails.networkInfo.networkTitle
		 */
		fun fromAddress(
			address: Address,
			base58: String,
			networkTitle: String
		): KeyCardModel =
			KeyCardModel(
				network = networkTitle.replaceFirstChar {
					if (it.isLowerCase()) it.titlecase() else it.toString()
				},
				cardBase = KeyCardModelBase.fromAddress(address, base58)
			)

		fun createStub() = KeyCardModel(
			network = "Polkadot",
			cardBase = KeyCardModelBase.createStub()
		)
	}
}


data class KeyCardModelBase(
	val base58: String,
	val path: String,
	val identIcon: ImageContent,
	val seedName: String,
	val hasPassword: Boolean = false,
	val multiselect: Boolean? = null,
) {
	companion object {

		fun fromKeyModel(model: KeyModel): KeyCardModelBase =
			KeyCardModelBase(
				base58 = model.base58,
				path = model.path,
				identIcon = model.identicon,
				seedName = model.seedName,
				hasPassword = model.hasPwd,
			)

		/**
		 * @param networkTitle probably from keyDetails.networkInfo.networkTitle
		 */
		fun fromAddress(
			address_card: MAddressCard,
		): KeyCardModelBase =
			KeyCardModelBase(
				base58 = address_card.base58,
				path = address_card.address.path,
				hasPassword = address_card.address.hasPwd,
				identIcon = address_card.address.identicon.toImageContent(),
				seedName = address_card.address.seedName,
			)

		fun fromAddress(
			address: Address,
			base58: String,
		): KeyCardModelBase =
			KeyCardModelBase(
				base58 = base58,
				path = address.path,
				hasPassword = address.hasPwd,
				identIcon = address.identicon.toImageContent(),
				seedName = address.seedName,
				multiselect = false,
			)

		fun createStub() = KeyCardModelBase(
			base58 = "5F3sa2TJAWMqDhXG6jhV4N8ko9SxwGy8TpaNS1repo5EYjQX",
			path = "//polkadot//path",
			identIcon = PreviewData.exampleIdenticonPng,
			seedName = "Seed Name",
			hasPassword = false,
			multiselect = null,
		)
	}
}


@Preview(
	name = "day",
	uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true,
)
@Preview(
	name = "dark theme",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	backgroundColor = 0xFFFFFFFF
)
@Composable
private fun PreviewKeyCard() {
	SignerNewTheme {
		KeyCard(model = KeyCardModel.createStub())
	}
}

@Preview(
	name = "day",
	uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true,
)
@Preview(
	name = "dark theme",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	backgroundColor = 0xFFFFFFFF
)
@Composable
private fun PreviewKeySeedCard() {
	SignerNewTheme {
		KeySeedCard(
			seedTitle = "Seed title",
			base58 = KeyCardModel.createStub().cardBase.base58,
		)
	}
}

@Preview(
	name = "day",
	uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true,
)
@Preview(
	name = "dark theme",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	backgroundColor = 0xFFFFFFFF
)
@Composable
private fun PreviewNetworkLabel() {
	SignerNewTheme {
		Box(Modifier.size(width = 100.dp, height = 500.dp)) {
			NetworkLabel("Polkadot")
		}
	}
}
