package io.parity.signer.screens.keysets.export

import android.content.res.Configuration
import androidx.annotation.StringRes
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.MutableState
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.pluralStringResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.ScreenHeaderClose
import io.parity.signer.components.items.KeySetItemMultiselect
import io.parity.signer.models.Callback
import io.parity.signer.models.KeySetModel
import io.parity.signer.models.KeySetsSelectModel
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.*

/**
 * Default main screen with list Seeds/root keys
 */
@OptIn(ExperimentalComposeUiApi::class)
@Composable
fun KeySetsSelectExportScreenContent(
	model: KeySetsSelectModel,
	selected: MutableState<Set<KeySetModel>>,
	onClose: Callback,
	onExportSelected: Callback,
	onExportAll: Callback,
) {
	Column(Modifier.background(MaterialTheme.colors.background)) {
		ScreenHeaderClose(
			if (selected.value.isEmpty()) {
				stringResource(R.string.key_set_multiselect_title_none_selected)
			} else {
				pluralStringResource(
					id = R.plurals.key_set_multiselect_title_some_selected,
					count = selected.value.size,
					selected.value.size,
				)
			},
			onClose = onClose,
		)
		LazyColumn(
			contentPadding = PaddingValues(horizontal = 12.dp),
			verticalArrangement = Arrangement.spacedBy(10.dp),
			modifier = Modifier.weight(1f),
		) {
			val cards = model.keys
			items(cards.size) { i ->
				KeySetItemMultiselect(
					model = cards[i],
					isSelected = selected.value.contains(cards[i])
				) { checked, model ->
					if (checked) selected.value += model else selected.value -= model
				}
			}
		}
		Row(
			modifier = Modifier
				.height(48.dp)
				.background(MaterialTheme.colors.backgroundSecondary),
			verticalAlignment = Alignment.CenterVertically,
		) {
			ClickableLabel(
				stringId = R.string.key_set_export_all_label,
				isEnabled = true,
				modifier = Modifier.padding(start = 16.dp, end = 16.dp),
				onClick = onExportAll,
			)
			Spacer(modifier = Modifier.weight(1f))
			ClickableLabel(
				stringId = R.string.key_set_export_selected_label,
				isEnabled = selected.value.isNotEmpty(),
				modifier = Modifier.padding(start = 16.dp, end = 16.dp),
				onClick = onExportSelected,
			)
		}
	}
}

@Composable
fun ClickableLabel(
	@StringRes stringId: Int,
	isEnabled: Boolean,
	modifier: Modifier = Modifier,
	onClick: () -> Unit,
) {
	val modifier = if (isEnabled) {
		modifier.clickable(onClick = onClick)
	} else {
		modifier
	}
	Text(
		text = stringResource(id = stringId),
		color = if (isEnabled) {
			MaterialTheme.colors.pink300
		} else {
			MaterialTheme.colors.textDisabled
		},
		style = TypefaceNew.TitleS,
		modifier = modifier,
	)
}


@Preview(
	name = "light", group = "general", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "general",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewKeySetsSelectExportScreen() {
	val keys = mutableListOf(
		KeySetModel(
			"first seed name",
			PreviewData.exampleIdenticon,
			1.toUInt()
		),
		KeySetModel(
			"second seed name",
			PreviewData.exampleIdenticon,
			3.toUInt()
		),
	)
	repeat(30) {
		keys.add(
			KeySetModel(
				"second seed name",
				PreviewData.exampleIdenticon,
				3.toUInt()
			)
		)
	}
	val mockModel = KeySetsSelectModel(keys)
	SignerNewTheme {
		Box(modifier = Modifier.size(350.dp, 550.dp)) {
			val selected = remember<MutableState<Set<KeySetModel>>> {
				mutableStateOf(emptySet())
			}
			KeySetsSelectExportScreenContent(mockModel, selected, {}, {}, {})
		}
	}
}
