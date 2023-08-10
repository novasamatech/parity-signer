package io.parity.signer.screens.keysetdetails.export

import SignerCheckbox
import android.content.res.Configuration
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.pluralStringResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import io.parity.signer.R
import io.parity.signer.components.IdentIconWithNetwork
import io.parity.signer.components.base.BottomSheetHeader
import io.parity.signer.components.base.ClickableLabel
import io.parity.signer.components.base.ScreenHeaderClose
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.components.sharedcomponents.KeyPath
import io.parity.signer.domain.*
import io.parity.signer.screens.keysetdetails.items.KeyDerivedItemMultiselect
import io.parity.signer.screens.keysetdetails.items.NetworkKeysExpandableMultiselect
import io.parity.signer.screens.keysetdetails.items.SeedKeyDetails
import io.parity.signer.ui.theme.*

/**
 * Single Seed/Key set is selected is it's details
 * For non-multiselect state,
 * For multiselec screen KeyManager is still used
 */
@Composable
fun KeySetDetailsMultiselectBottomSheet(
	model: KeySetDetailsModel,
	selected: MutableState<Set<String>>,
	onClose: Callback,
	onExportSelected: Callback,
	onExportAll: Callback,
) {
	Column {
		val keysToExport = selected.value.size + 1 // + root key
		BottomSheetHeader(
			pluralStringResource(
				id = R.plurals.key_export_title,
				count = keysToExport,
				keysToExport,
			),
			onCloseClicked = onClose,
		)
		SignerDivider(sidePadding = 24.dp)
		Column(
			modifier = Modifier
				.padding(horizontal = 8.dp)
				.verticalScroll(rememberScrollState()),
			verticalArrangement = Arrangement.spacedBy(4.dp),
		) {
			//seed - key set
			model.root?.let {
				DisabledSelectedKey(it.base58)
			}
			for (item in model.keysAndNetwork) {
				KeyDerivedItemMultiselect(
					model = item.key,
					networkLogo = item.network.networkLogo,
					isSelected = selected.value.contains(item.key.addressKey),
					onClick = { isSelected, address ->
						if (isSelected) selected.value += address else selected.value -= address
					},
				)
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
				isEnabled = true,
				modifier = Modifier.padding(start = 16.dp, end = 16.dp),
				onClick = onExportSelected,
			)
		}
	}
}

@Composable
private fun DisabledSelectedKey(base58: String) {
	Row(
		verticalAlignment = Alignment.CenterVertically,
		modifier = Modifier
			.padding(vertical = 16.dp)
	) {
		Column(Modifier.weight(1f)) {
			Text(
				text = base58.abbreviateString(BASE58_STYLE_ABBREVIATE),
				color = MaterialTheme.colors.textDisabled,
				style = SignerTypeface.BodyL,
				modifier = Modifier.padding(horizontal = 16.dp),
			)
		}
		SignerCheckbox(
			isChecked = true,
			checkedColor = MaterialTheme.colors.textDisabled,
			modifier = Modifier
				.padding(8.dp)
				.padding(horizontal = 8.dp)
		) {
			//no react on click
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
			KeySetDetailsMultiselectBottomSheet(
				stabModel,
				state,
				{},
				{},
				{},
			)
		}
	}
}
