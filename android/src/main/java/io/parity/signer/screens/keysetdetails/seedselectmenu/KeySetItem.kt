package io.parity.signer.screens.keysetdetails.seedselectmenu

import android.content.res.Configuration
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.pluralStringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.CheckIcon
import io.parity.signer.components.networkicon.IdentIconImage
import io.parity.signer.domain.Callback
import io.parity.signer.domain.KeySetModel
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.textSecondary

@Composable
fun KeySetItem(
	model: KeySetModel,
	isSelected: Boolean,
	onClick: Callback,
) {
	Surface(
		shape = RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius)),
		modifier = Modifier.clickable(onClick = onClick),
	) {
		Row(
			verticalAlignment = Alignment.CenterVertically,
		) {
			IdentIconImage(
				identicon = model.identicon,
				modifier = Modifier.padding(
					top = 16.dp, bottom = 16.dp, start = 16.dp, end = 12.dp
				),
				size = 36.dp
			)
			Column(Modifier.weight(1f)) {
				Text(
					text = model.seedName,
					color = MaterialTheme.colors.primary,
					style = SignerTypeface.TitleS,
				)
				if (model.derivedKeysCount > 0.toUInt()) {
					Spacer(modifier = Modifier.padding(top = 4.dp))
					Text(
						text = pluralStringResource(
							id = R.plurals.key_sets_item_derived_subtitle,
							count = model.derivedKeysCount.toInt(),
							model.derivedKeysCount.toInt(),
						),
						color = MaterialTheme.colors.textSecondary,
						style = SignerTypeface.BodyM,
					)
				}
			}
			if (isSelected) {
				CheckIcon(modifier = Modifier.padding(end = 16.dp))
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
		KeySetItem(
			model = KeySetModel.createStub("My special key set", 2),
			isSelected = true,
			onClick = {},
		)
	}
}
