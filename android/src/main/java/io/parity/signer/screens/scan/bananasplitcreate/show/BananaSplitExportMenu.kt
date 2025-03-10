package io.parity.signer.screens.scan.bananasplitcreate.show

import android.content.res.Configuration
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.outlined.Password
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.BottomSheetConfirmDialog
import io.parity.signer.components.base.SecondaryButtonWide
import io.parity.signer.domain.Callback
import io.parity.signer.screens.keydetails.MenuItemForBottomSheet
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.red400


@Composable
fun BananaSplitExportRemoveConfirmBottomSheet(
	onCancel: Callback,
	onRemoveKeySet: Callback,
) {
	BottomSheetConfirmDialog(
		title = stringResource(R.string.banana_split_menu_remove_confirm_title),
		message = stringResource(R.string.banana_split_menu_remove_confirm_description),
		ctaLabel = stringResource(R.string.remove_key_set_confirm_cta),
		isCtaDangerous = true,
		onCancel = onCancel,
		onCta = onRemoveKeySet,
	)
}

@Composable
fun BananaSplitExportMenuBottomSheet(
	onShowPassphrase: Callback,
	onRemoveBackup: Callback,
	onCancel: Callback,
) {
	val sidePadding = 24.dp
	Column(
		modifier = Modifier
			.fillMaxWidth()
			.padding(start = sidePadding, end = sidePadding, top = 8.dp),
	) {

		MenuItemForBottomSheet(
			Icons.Outlined.Password,
			label = stringResource(R.string.banana_split_menu_option_show_password),
			onclick = onShowPassphrase
		)

		MenuItemForBottomSheet(
			iconId = R.drawable.ic_backspace_28,
			label = stringResource(R.string.banana_split_menu_option_remove_backup),
			tint = MaterialTheme.colors.red400,
			onclick = onRemoveBackup
		)
		Spacer(modifier = Modifier.padding(bottom = 8.dp))
		SecondaryButtonWide(
			label = stringResource(R.string.generic_cancel),
			onClicked = onCancel
		)
		Spacer(modifier = Modifier.padding(bottom = 16.dp))
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
private fun PreviewBananaSplitExportBottomSheet() {
	SignerNewTheme {
		BananaSplitExportMenuBottomSheet(
		{}, {}, {},
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
private fun PreviewKBananaSplitExportRemoveConfirmBottomSheet() {
	SignerNewTheme {
		BananaSplitExportRemoveConfirmBottomSheet(
			{}, {},
		)
	}
}
