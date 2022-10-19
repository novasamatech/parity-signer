package io.parity.signer.components.items

import SignerCheckboxColors
import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.Checkbox
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ChevronRight
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.pluralStringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.IdentIcon
import io.parity.signer.screens.keysets.KeySetViewModel
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.*

@OptIn(ExperimentalComposeUiApi::class)
@Composable
fun KeySetItemMultiselect(
	model: KeySetViewModel,
	onClick: (Boolean, KeySetViewModel) -> Unit,
) {
	val checkedState = remember { mutableStateOf(false) }
	Surface(
		shape = RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius)),
		color = MaterialTheme.colors.backgroundSecondary,
		modifier = Modifier.clickable(onClick = {
			checkedState.value = !checkedState.value
			onClick(checkedState.value, model)
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
					style = TypefaceNew.LabelM,
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
						style = TypefaceNew.BodyM,
					)
				}
			}
			Checkbox(
				checked = checkedState.value,
				onCheckedChange = { c ->
					checkedState.value = !checkedState.value
					onClick(c, model)
				},
				colors = SignerCheckboxColors(),
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
private fun PreviewKeySetItemMultiselect() {
	SignerNewTheme {
		val model = KeySetViewModel(
			"My special key set",
			PreviewData.exampleIdenticon,
			2.toUInt()
		)
		KeySetItemMultiselect(
			model,
		) { _, _ -> }
	}
}
