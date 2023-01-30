package io.parity.signer.components.items

import SignerCheckbox
import android.content.res.Configuration
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.pluralStringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.IdentIcon
import io.parity.signer.domain.KeySetModel
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.*

@OptIn(ExperimentalComposeUiApi::class)
@Composable
fun KeySetItemMultiselect(
	model: KeySetModel,
	isSelected: Boolean = false,
	onClick: (Boolean, KeySetModel) -> Unit,
) {
	Surface(
		shape = RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius)),
		color = MaterialTheme.colors.backgroundSecondary,
		modifier = Modifier.clickable(onClick = {
			onClick(!isSelected, model)
		}),
	) {
		Row(
			verticalAlignment = Alignment.CenterVertically,
		) {
			IdentIcon(
				identicon = model.identicon, size = 36.dp,
				modifier = Modifier.padding(
					top = 16.dp,
					bottom = 16.dp,
					start = 16.dp,
					end = 12.dp
				)
			)
			Column(Modifier.weight(1f)) {
				Text(
					text = model.seedName,
					color = MaterialTheme.colors.primary,
					style = SignerTypeface.LabelM,
				)
				if (model.derivedKeysCount > 0.toUInt()) {
					Spacer(modifier = Modifier.padding(top = 4.dp))
					Text(
						text = pluralStringResource(
							id = R.plurals.key_sets_item_derived_subtitle,
							count = model.derivedKeysCount.toInt(),
							model.derivedKeysCount.toInt(),
						),
						color = MaterialTheme.colors.textDisabled,
						style = SignerTypeface.BodyM,
					)
				}
			}
			SignerCheckbox(
				isChecked = isSelected,
				modifier = Modifier
					.padding(end = 8.dp)
			) {
				onClick(!isSelected, model)
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
private fun PreviewKeySetItemMultiselect() {
	SignerNewTheme {
		val model = KeySetModel(
			"My special key set",
			PreviewData.exampleIdenticonPng,
			2.toUInt()
		)
		KeySetItemMultiselect(
			model,
		) { _, _ -> }
	}
}
