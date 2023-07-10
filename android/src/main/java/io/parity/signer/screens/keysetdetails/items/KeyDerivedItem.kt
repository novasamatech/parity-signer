package io.parity.signer.screens.keysetdetails.items

import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
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
import io.parity.signer.domain.BASE58_STYLE_ABBREVIATE
import io.parity.signer.domain.Callback
import io.parity.signer.domain.KeyAndNetworkModel
import io.parity.signer.domain.KeyModel
import io.parity.signer.domain.NetworkInfoModel
import io.parity.signer.domain.abbreviateString
import io.parity.signer.domain.conditional
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.textTertiary

@Composable
fun KeyDerivedItem(
	model: KeyModel,
	networkLogo: String,
	onClick: Callback? = {},
) {
	Surface(
		shape = RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius)),
		color = Color.Transparent,
		modifier = Modifier.conditional(onClick != null) {
			clickable(onClick = onClick ?: {})
		},
	) {
		Row(
			modifier = Modifier.padding(vertical = 8.dp, horizontal = 16.dp),
			verticalAlignment = Alignment.CenterVertically,
		) {
			IdentIconWithNetwork(
				identicon = model.identicon,
				networkLogoName = networkLogo,
				size = 36.dp,
				modifier = Modifier.padding(end = 12.dp),
			)
			Column(Modifier.weight(1f)) {
				if (model.wasImported == true) {
					Text(
						text = stringResource(R.string.dynamic_derivation_path_label),
						style = SignerTypeface.CaptionM,
						color = MaterialTheme.colors.textTertiary,
					)
				}
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
			if (onClick != null) {
				Image(
					imageVector = Icons.Filled.ChevronRight,
					contentDescription = null,
					colorFilter = ColorFilter.tint(MaterialTheme.colors.textTertiary),
					modifier = Modifier
						.padding(2.dp)// because it's 28 not 32pd
						.padding(start = 12.dp)
						.size(28.dp)
				)
			}
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
		Column {
			KeyDerivedItem(
				KeyModel.createStub(wasImported = false),
				"kusama"
			)
			KeyDerivedItem(
				KeyModel.createStub(wasImported = true),
				"kusama",
			)
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
