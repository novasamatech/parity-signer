package io.parity.signer.screens.keysetdetails.export

import android.content.res.Configuration
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.pluralStringResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.ScreenHeaderClose
import io.parity.signer.components.items.KeyDerivedItemMultiselect
import io.parity.signer.models.*
import io.parity.signer.screens.keysetdetails.SeedKeyViewItem
import io.parity.signer.screens.keysets.export.ClickableLabel
import io.parity.signer.ui.theme.*

/**
 * Single Seed/Key set is selected is it's details
 * For non-multiselect state,
 * For multiselec screen KeyManager is still used
 */
//todo dmitry finish this screen
@OptIn(ExperimentalComposeUiApi::class)
@Composable
fun KeySetDetailsMultiselectScreen(
	model: KeySetDetailsModel,
	selected: MutableState<Set<String>>,
	onClose: Callback,
	onExportSelected: Callback,
	onExportAll: Callback,
) {
	Column {
		ScreenHeaderClose(
			if (selected.value.isEmpty()) {
				stringResource(R.string.key_set_details_multiselect_title_none_selected)
			} else {
				pluralStringResource(
					id = R.plurals.key_export_title,
					count = selected.value.size,
					selected.value.size,
				)
			},
			onClose = onClose,
		)
		Column(
			modifier = Modifier
				.weight(1f)
				.verticalScroll(rememberScrollState())
		) {
			//seed
			SeedKeyViewItem(model.root, null) //todo dmitry make grey colors
			//filter row
			Row(
				modifier = Modifier.padding(horizontal = 24.dp),
				verticalAlignment = Alignment.CenterVertically
			) {
				Text(
					text = stringResource(R.string.key_sets_details_screem_derived_subtitle),
					color = MaterialTheme.colors.textDisabled,
					style = TypefaceNew.BodyM,
					modifier = Modifier.weight(1f),
				)
				Icon(
					painter = painterResource(id = R.drawable.ic_tune_28),
					contentDescription = stringResource(R.string.key_sets_details_screem_filter_icon_description),
					modifier = Modifier
						.size(28.dp),
					tint = MaterialTheme.colors.textDisabled,
				)
			}
			for (key in model.keys) {
				KeyDerivedItemMultiselect(
					model = key,
					isSelected = selected.value.contains(key.addressKey),
				) { isSelected, key ->
					if (isSelected) selected.value += key else selected.value -= key
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
				isEnabled = true, //selected.value.isNotEmpty(),
				modifier = Modifier.padding(start = 16.dp, end = 16.dp),
				onClick = onExportSelected,
			)
		}
	}
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
private fun PreviewKeySetDetailsMultiselectScreen() {

	val state = remember { mutableStateOf(AlertState.Active) }
	val mockModel = KeySetDetailsModel.createStub()
	SignerNewTheme {
		Box(modifier = Modifier.size(350.dp, 550.dp)) {
			KeySetDetailsMultiselectScreen(mockModel, EmptyNavigator(), state)
		}
	}
}
