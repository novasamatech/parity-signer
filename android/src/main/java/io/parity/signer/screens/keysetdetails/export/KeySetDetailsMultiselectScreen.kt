package io.parity.signer.screens.keysetdetails.export

import android.content.res.Configuration
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.*
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ChevronRight
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.pluralStringResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.PrimaryButtonBottomSheet
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.components.base.ScreenHeaderClose
import io.parity.signer.components.exposesecurity.ExposedIcon
import io.parity.signer.components.items.KeyDerivedItem
import io.parity.signer.models.*
import io.parity.signer.screens.keysets.export.ClickableLabel
import io.parity.signer.ui.theme.*
import io.parity.signer.uniffi.Action

/**
 * Single Seed/Key set is selected is it's details
 * For non-multiselect state,
 * For multiselec screen KeyManager is still used
 */
//todo dmitry finish this screen
@Composable
fun KeySetDetailsMultiselectScreen(
	model: KeySetDetailsModel,
	navigator: Navigator,
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
			SeedKeyViewItem(model.root) {
				navigator.navigate(Action.SELECT_KEY, model.root.addressKey)
			}
			//filter row
			Row(
				modifier = Modifier.padding(horizontal = 24.dp),
				verticalAlignment = Alignment.CenterVertically
			) {
				Text(
					text = stringResource(R.string.key_sets_details_screem_derived_subtitle),
					color = MaterialTheme.colors.textTertiary,
					style = TypefaceNew.BodyM,
					modifier = Modifier.weight(1f),
				)
				Icon(
					painter = painterResource(id = R.drawable.ic_tune_28),
					contentDescription = stringResource(R.string.key_sets_details_screem_filter_icon_description),
					modifier = Modifier
						.clickable { navigator.navigate(Action.NETWORK_SELECTOR, "") }
						.size(28.dp),
					tint = MaterialTheme.colors.textTertiary,
				)
			}
			for (key in model.keys) {
				KeyDerivedItem(model = key) {
					navigator.navigate(Action.SELECT_KEY, key.addressKey)
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
				onClick = {},//todo dmitry
			)
			Spacer(modifier = Modifier.weight(1f))
			ClickableLabel(
				stringId = R.string.key_set_export_selected_label,
				isEnabled = true, //selected.value.isNotEmpty(),
				modifier = Modifier.padding(start = 16.dp, end = 16.dp),
				onClick = {}//todo dmitry,
			)
		}
	}
}

@Composable
private fun SeedKeyViewItem(
	seedKeyModel: KeysModel,
	onClick: Callback,
) {
	Row(
		modifier = Modifier
			.padding(top = 16.dp, bottom = 16.dp, start = 24.dp)
			.clickable(onClick = onClick),
		verticalAlignment = Alignment.CenterVertically,
	) {
		Column(Modifier.weight(1f)) {
			Text(
				text = seedKeyModel.seedName,
				color = MaterialTheme.colors.primary,
				style = TypefaceNew.TitleL,
			)
			Text(
				text = seedKeyModel.base58.abbreviateString(8),
				color = MaterialTheme.colors.textTertiary,
				style = TypefaceNew.BodyM,
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
