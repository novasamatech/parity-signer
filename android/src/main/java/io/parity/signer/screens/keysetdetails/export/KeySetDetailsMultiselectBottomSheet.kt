package io.parity.signer.screens.keysetdetails.export

import SignerCheckbox
import android.content.res.Configuration
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.MutableState
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.pluralStringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.BottomSheetHeader
import io.parity.signer.components.base.ClickableLabel
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.domain.BASE58_STYLE_ABBREVIATE
import io.parity.signer.domain.Callback
import io.parity.signer.domain.KeySetDetailsModel
import io.parity.signer.domain.abbreviateString
import io.parity.signer.screens.keysetdetails.items.KeyDerivedItemMultiselect
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.textDisabled

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
	Column(
		modifier = Modifier
			.verticalScroll(rememberScrollState())
	) {
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
				.padding(horizontal = 8.dp),
		) {
			//seed - key set
			DisabledSelectedKey(model.root.base58)
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
				.height(48.dp),
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
			.padding(vertical = 8.dp)
	) {
		Column(Modifier.weight(1f)) {
			Text(
				text = base58.abbreviateString(BASE58_STYLE_ABBREVIATE),
				color = MaterialTheme.colors.textDisabled,
				style = SignerTypeface.BodyL,
				modifier = Modifier.padding(start = 24.dp, end = 8.dp),
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
private fun PreviewDisabledSelectedKey() {
	SignerNewTheme {
		DisabledSelectedKey("sdfsdfsdfsdfsdfsdf")
	}
}
