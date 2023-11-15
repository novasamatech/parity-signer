package io.parity.signer.components.sharedcomponents

import android.content.res.Configuration
import androidx.compose.animation.animateContentSize
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.InlineTextContent
import androidx.compose.foundation.text.appendInlineContent
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.KeyboardArrowDown
import androidx.compose.material.icons.filled.Lock
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.Placeholder
import androidx.compose.ui.text.PlaceholderVerticalAlign
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.buildAnnotatedString
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.TextUnit
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import io.parity.signer.R
import io.parity.signer.components.networkicon.IdentIconWithNetwork
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.components.networkicon.IdentIconImage
import io.parity.signer.domain.BASE58_STYLE_ABBREVIATE
import io.parity.signer.domain.KeyDetailsModel
import io.parity.signer.domain.KeyModel
import io.parity.signer.domain.NetworkInfoModel
import io.parity.signer.domain.abbreviateString
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.*
import io.parity.signer.uniffi.Address
import io.parity.signer.uniffi.Identicon
import io.parity.signer.uniffi.MAddressCard
import io.parity.signer.uniffi.MKeyDetails
import java.util.*


@Composable
fun KeyCard(model: KeyCardModel) {
	Row(
		Modifier
			.fillMaxWidth()
			.padding(16.dp),
		verticalAlignment = Alignment.CenterVertically,
	) {
		IdentIconWithNetwork(
			identicon = model.cardBase.identIcon,
			networkLogoName = model.network,
			size = 36.dp,
			modifier = Modifier.padding(end = 12.dp)
		)
		Column() {
			if (model.cardBase.path.isNotEmpty()) {
				KeyPath(model.cardBase.path, model.cardBase.hasPassword)
			}
			Text(
				model.cardBase.base58.abbreviateString(BASE58_STYLE_ABBREVIATE),
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.BodyL,
				maxLines = 1,
			)
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
fun KeySeedCard(identicon: Identicon, base58: String) {
	Row(
		Modifier
			.fillMaxWidth()
			.padding(16.dp),
		verticalAlignment = Alignment.CenterVertically,
	) {
		IdentIconImage(
			identicon = identicon,
			modifier = Modifier.padding(end = 12.dp),
			size = 36.dp,
		)
		Text(
			base58.abbreviateString(BASE58_STYLE_ABBREVIATE),
			color = MaterialTheme.colors.textTertiary,
			style = SignerTypeface.BodyL,
			maxLines = 1,
		)
	}
}

@Composable
fun KeyPath(
	path: String, hasPassword: Boolean,
	textStyle: TextStyle = SignerTypeface.CaptionM,
	iconSize: TextUnit = 14.sp,
	textColor: Color = MaterialTheme.colors.textSecondary,
	iconColor: Color = MaterialTheme.colors.textSecondary,
) {
	val imageId = "iconId$$"
	val annotatedString = buildAnnotatedString {
		append(path)
		if (hasPassword) {
			append(" •••• ")
			appendInlineContent(id = imageId)
		}
	}
	val inlineContentMap = mapOf(
		imageId to InlineTextContent(
			Placeholder(
				iconSize,
				iconSize,
				PlaceholderVerticalAlign.TextCenter
			)
		) {
			Icon(
				Icons.Default.Lock,
				contentDescription = stringResource(R.string.description_locked_icon),
				tint = iconColor,
			)
		}
	)

	Text(
		annotatedString,
		inlineContent = inlineContentMap,
		color = textColor,
		style = textStyle,
	)
}

@Composable
fun ShowBase58Collapsible(
	base58: String,
	modifier: Modifier = Modifier,
) {
	val expanded = remember { mutableStateOf(false) }
	Box(
		contentAlignment = Alignment.CenterStart,
		modifier = modifier
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
			ShowBase58Collapsed(base58)
		}
	}
}

@Composable
fun ShowBase58Collapsed(
	base58: String,
	modifier: Modifier = Modifier,
) {
	Row(modifier = modifier) {
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


data class KeyCardModel(
	val network: String,
	val cardBase: KeyCardModelBase,
) {
	companion object {

		fun fromKeyModel(model: KeyModel, network: NetworkInfoModel): KeyCardModel =
			KeyCardModel(
				network = network.networkTitle.replaceFirstChar {
					if (it.isLowerCase()) it.titlecase() else it.toString()
				},
				cardBase = KeyCardModelBase.fromKeyModel(
					model,
					networkLogo = network.networkLogo
				)
			)

		/**
		 * @param networkTitle probably from keyDetails.networkInfo.networkTitle
		 */
		fun fromAddress(
			address: Address,
			base58: String,
			network: NetworkInfoModel
		): KeyCardModel =
			KeyCardModel(
				network = network.networkTitle.replaceFirstChar {
					if (it.isLowerCase()) it.titlecase() else it.toString()
				},
				cardBase = KeyCardModelBase.fromAddress(
					address,
					base58,
					network.networkLogo
				)
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
	val identIcon: Identicon,
	val networkLogo: String?,
	val seedName: String,
	val hasPassword: Boolean = false,
	val multiselect: Boolean? = null,
) {
	companion object {

		fun fromKeyModel(model: KeyModel, networkLogo: String?): KeyCardModelBase =
			KeyCardModelBase(
				base58 = model.base58,
				path = model.path,
				identIcon = model.identicon,
				seedName = model.seedName,
				hasPassword = model.hasPwd,
				networkLogo = networkLogo,
			)

		/**
		 * @param networkTitle probably from keyDetails.networkInfo.networkTitle
		 */
		fun fromAddress(
			address_card: MAddressCard, networkLogo: String?
		): KeyCardModelBase =
			KeyCardModelBase(
				base58 = address_card.base58,
				path = address_card.address.path,
				hasPassword = address_card.address.hasPwd,
				identIcon = address_card.address.identicon,
				seedName = address_card.address.seedName,
				networkLogo = networkLogo,
			)

		fun fromAddress(
			address: Address,
			base58: String,
			networkLogo: String?,
		): KeyCardModelBase =
			KeyCardModelBase(
				base58 = base58,
				path = address.path,
				hasPassword = address.hasPwd,
				identIcon = address.identicon,
				seedName = address.seedName,
				networkLogo = networkLogo,
				multiselect = false,
			)

		fun createStub() = KeyCardModelBase(
			base58 = "5F3sa2TJAWMqDhXG6jhV4N8ko9SxwGy8TpaNS1repo5EYjQX",
			path = "//polkadot//path",
			identIcon = PreviewData.Identicon.dotIcon,
			seedName = "Seed Name",
			networkLogo = "kusama",
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
	val model = KeyCardModel.createStub()
	SignerNewTheme {
		Column() {
			KeyCard(model = model)
			SignerDivider()
			KeyCard(
				model = model.copy(
					cardBase = model.cardBase.copy(
						path = "//kusama//some//very_long_path//somesomesome",
						hasPassword = true,
					)
				)
			)
			SignerDivider()
			KeyCard(
				model = model.copy(
					cardBase = model.cardBase.copy(path = "")
				)
			)
		}
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
		NetworkLabel("Polkadot")
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
			identicon = PreviewData.Identicon.jdenticonIcon,
			base58 = KeyCardModel.createStub().cardBase.base58,
		)
	}
}

