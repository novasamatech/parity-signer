package io.parity.signer.screens.keysetdetails.seedselectmenu.old

import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ChevronRight
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.pluralStringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.networkicon.NetworkIcon
import io.parity.signer.domain.KeySetModel
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.*

@Composable
fun KeySetItem2(
	model: KeySetModel,
	onClick: () -> Unit = {},
) {
	val background = MaterialTheme.colors.backgroundSecondary
	Surface(
		shape = RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius)),
		color = background,
		modifier = Modifier.clickable(onClick = onClick),
	) {
		Column() {
			Text(
				text = pluralStringResource(
					id = R.plurals.key_sets_item_derived_subtitle,
					count = model.derivedKeysCount.toInt(),
					model.derivedKeysCount.toInt(),
				),
				color = MaterialTheme.colors.textTertiary,
				style = SignerTypeface.BodyM,
				modifier = Modifier
					.padding(horizontal = 16.dp)
					.padding(top = 16.dp)
			)
			//title
			Row(
				verticalAlignment = Alignment.CenterVertically,
				modifier = Modifier
					.padding(horizontal = 16.dp)
					.padding(top = 4.dp, bottom = 24.dp),
			) {

				Column(Modifier.weight(1f)) {
					Text(
						text = model.seedName,
						color = MaterialTheme.colors.primary,
						style = SignerTypeface.TitleL,
					)
				}
				Image(
					imageVector = Icons.Filled.ChevronRight,
					contentDescription = null,
					colorFilter = ColorFilter.tint(MaterialTheme.colors.textDisabled),
					modifier = Modifier
						.padding(end = 8.dp)
						.size(28.dp)
				)
			}
			//icons
			if (model.usedInNetworks.isNotEmpty()) {
				Row(
					modifier = Modifier
						.padding(horizontal = 12.dp)
						.padding(bottom = 16.dp)
				) {
					model.usedInNetworks.take(7).forEachIndexed { index, network ->
						Box(
							modifier = Modifier
								.offset(x = (-10).dp * index)
								.background(background, CircleShape)
								.size(40.dp),
							contentAlignment = Alignment.Center
						) {
							NetworkIcon(
								networkLogoName = network, size = 32.dp,
							)
						}
					}
				}
			}
		}
	}
}


@Preview(
	name = "light", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewKeySetItem() {
	SignerNewTheme {
		KeySetItem2(
			KeySetModel(
				"My special key set",
				PreviewData.Identicon.dotIcon,
				listOf("westend", "some", "polkadot", "www", "kusama"),
				2.toUInt()
			)
		)
	}
}

@Preview(
	name = "light", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewKeySetItemEmpty() {
	SignerNewTheme {
		KeySetItem2(
			KeySetModel(
				"My special key set",
				PreviewData.Identicon.dotIcon,
				emptyList(),
				0.toUInt(),
			)
		)
	}
}
