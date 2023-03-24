package io.parity.signer.screens.keysetdetails.export

import android.content.res.Configuration
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.pluralStringResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.ScreenHeaderClose
import io.parity.signer.domain.*
import io.parity.signer.screens.keysetdetails.items.NetworkKeysExpandableMultiselect
import io.parity.signer.screens.keysetdetails.items.SeedKeyDetails
import io.parity.signer.screens.keysets.export.ClickableLabel
import io.parity.signer.ui.theme.*

/**
 * Single Seed/Key set is selected is it's details
 * For non-multiselect state,
 * For multiselec screen KeyManager is still used
 */
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
				.padding(horizontal = 8.dp)
				.verticalScroll(rememberScrollState()),
			verticalArrangement = Arrangement.spacedBy(4.dp),
			) {
			//seed - key set
			model.root?.let {
				SeedKeyDetails(
					model = it,
					Modifier.padding(horizontal = 24.dp, vertical = 8.dp)
				)
			}

			val models = model.keysAndNetwork.groupBy { it.network }
			for (networkAndKeys in models.entries) {
				NetworkKeysExpandableMultiselect(
					network = networkAndKeys.key.toNetworkModel(),
					keys = networkAndKeys.value
						.map { it.key }
						.sortedBy { it.path },
					selectedKeysAdr = selected.value,
				) { isSelected, keyAdr ->
					if (isSelected) selected.value += keyAdr else selected.value -= keyAdr
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

	val stabModel = KeySetDetailsModel.createStub()
	val state =
		remember { mutableStateOf(setOf(stabModel.keysAndNetwork[1].key.addressKey)) }
	SignerNewTheme {
		Box(modifier = Modifier.size(350.dp, 550.dp)) {
			KeySetDetailsMultiselectScreen(stabModel, state, {}, {}, {})
		}
	}
}
