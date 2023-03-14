package io.parity.signer.screens.keysetdetails.items

import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.Icon
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
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.IdentIcon
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.components.sharedcomponents.NetworkLabel
import io.parity.signer.domain.*
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.textDisabled
import io.parity.signer.ui.theme.textTertiary

@Composable
fun KeyDerivedItem(
	model: KeyModel,
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
			IdentIcon(
				identicon = model.identicon, size = 36.dp, modifier = Modifier.padding(
					top = 16.dp,
					bottom = 16.dp,
					start = 16.dp,
					end = 12.dp
				)
			)
			Column(Modifier.weight(1f)) {
				if (model.path.isNotEmpty() || model.hasPwd) {
					Row(verticalAlignment = Alignment.CenterVertically) {
						Text(
							text = model.path,
							color = MaterialTheme.colors.textTertiary,
							style = SignerTypeface.CaptionM,
						)
						if (model.hasPwd) {
							Icon(
								painterResource(id = R.drawable.ic_lock_16),
								contentDescription = stringResource(R.string.key_lock_item),
								tint = MaterialTheme.colors.textTertiary,
								modifier = Modifier.padding(start = 8.dp)
							)
						}
					}
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
		IdentIcon(
			identicon = model.key.identicon,
			size = 36.dp,
			modifier = Modifier.padding(
				top = 16.dp,
				bottom = 16.dp,
				start = 24.dp,
				end = 12.dp
			)
		)
		if (model.key.path.isNotEmpty()) {
			Text(
				text = model.key.path,
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.LabelM,
			)
			if (model.key.hasPwd) {
				Icon(
					painterResource(id = R.drawable.ic_lock_16),
					contentDescription = stringResource(R.string.key_lock_item),
					tint = MaterialTheme.colors.textTertiary,
					modifier = Modifier.padding(start = 8.dp)
				)
			}
		} else {
			Text(
				text = stringResource(R.string.derivation_key_empty_path_placeholder),
				color = MaterialTheme.colors.textTertiary,
				style = SignerTypeface.LabelM,
			)
		}
		Spacer(modifier = Modifier.weight(1f))
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
			KeyModel.createStub()
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
		}
	}
}
