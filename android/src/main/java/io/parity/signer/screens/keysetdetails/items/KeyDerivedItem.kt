package io.parity.signer.screens.keysetdetails.items

import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ChevronRight
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import io.parity.signer.R
import io.parity.signer.components.IdentIconWithNetwork
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.components.sharedcomponents.KeyPath
import io.parity.signer.components.sharedcomponents.NetworkLabel
import io.parity.signer.domain.*
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.textDisabled
import io.parity.signer.ui.theme.textTertiary

@Composable
fun KeyDerivedItem(
	model: KeyModel,
	network: String,
	onClick: () -> Unit = {},
) {
	Surface(
		shape = RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius)),
		color = Color.Transparent,
		modifier = Modifier.clickable(onClick = onClick),
	) {
		Row(
			verticalAlignment = Alignment.CenterVertically,
		) {
			IdentIconWithNetwork(
				identicon = model.identicon, networkLogoName = network,
				size = 36.dp, modifier = Modifier.padding(
					top = 16.dp,
					bottom = 16.dp,
					start = 16.dp,
					end = 12.dp
				)
			)
			Column(Modifier.weight(1f)) {
				if (model.path.isNotEmpty() || model.hasPwd) {
					KeyPath(
						path = model.path,
						hasPassword = model.hasPwd,
						textStyle = SignerTypeface.CaptionM,
						iconSize = 16.sp,
						textColor = MaterialTheme.colors.textTertiary,
						iconColor = MaterialTheme.colors.textTertiary,
					)
					Spacer(modifier = Modifier.padding(top = 4.dp))
				}
				Text(
					text = model.base58.abbreviateString(BASE58_STYLE_ABBREVIATE),
					color = MaterialTheme.colors.primary,
					style = SignerTypeface.BodyL,
				)
			}
			Image(
				imageVector = Icons.Filled.ChevronRight,
				contentDescription = null,
				colorFilter = ColorFilter.tint(MaterialTheme.colors.textDisabled),
				modifier = Modifier
                    .padding(end = 16.dp)
                    .size(28.dp)
			)
		}
	}
}

@Composable
fun SlimKeyItem(model: KeyAndNetworkModel) {
	Row(
		modifier = Modifier.fillMaxWidth(1f),
		verticalAlignment = Alignment.CenterVertically,
	) {
		IdentIconWithNetwork(
			identicon = model.key.identicon,
			networkLogoName = model.network.networkLogo,
			size = 36.dp,
			modifier = Modifier.padding(
				top = 16.dp,
				bottom = 16.dp,
				start = 24.dp,
				end = 12.dp,
			)
		)
		Box(modifier = Modifier.weight(1f)) {
			if (model.key.path.isNotEmpty()) {
				KeyPath(
					path = model.key.path,
					hasPassword = model.key.hasPwd,
					textColor = MaterialTheme.colors.primary,
					iconSize = 16.sp,
					iconColor = MaterialTheme.colors.textTertiary,
					textStyle = SignerTypeface.LabelM,
				)
			} else {
				Text(
					text = stringResource(R.string.derivation_key_empty_path_placeholder),
					color = MaterialTheme.colors.textTertiary,
					style = SignerTypeface.LabelM,
				)
			}
		}
		NetworkLabel(
			networkName = model.network.networkTitle,
			modifier = Modifier.padding(end = 24.dp, start = 8.dp)
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
private fun PreviewKeyDerivedItem() {
	SignerNewTheme {
		KeyDerivedItem(
			KeyModel.createStub(),
			"kusama"
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
private fun PreviewSlimKeyItem() {
	val model = KeyAndNetworkModel(
		key = KeyModel.createStub(),
		network = NetworkInfoModel.createStub()
	)
	SignerNewTheme {
		Column {
			SlimKeyItem(model)
			SignerDivider()
			SlimKeyItem(model.copy(key = model.key.copy(path = "")))
			SignerDivider()
			SlimKeyItem(
				model.copy(
					key = model.key.copy(
						path = "//kusama//some//very_long_path//somesomesome",
						hasPwd = true,
					)
				)
			)
		}
	}
}
