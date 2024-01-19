package io.parity.signer.screens.settings.logs.logslist

import android.content.res.Configuration
import timber.log.Timber
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.fragment.app.FragmentActivity
import io.parity.signer.R
import io.parity.signer.components.base.BottomSheetConfirmDialog
import io.parity.signer.components.base.SecondaryButtonWide
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.*
import io.parity.signer.screens.keydetails.MenuItemForBottomSheet
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.red400
import kotlinx.coroutines.launch

private const val TAG = "LogsMenu"

@Composable
fun LogsMenuGeneral(
	onCreateComment: Callback,
	onDeleteClicked: Callback,
	onCancel: Callback,
) {
	val sidePadding = 24.dp
	Column(
		modifier = Modifier
			.fillMaxWidth()
			.padding(start = sidePadding, end = sidePadding, top = 8.dp),
	) {

		MenuItemForBottomSheet(
			iconId = R.drawable.ic_add_28,
			label = stringResource(R.string.menu_option_add_note),
			onclick = onCreateComment,
		)

		MenuItemForBottomSheet(
			iconId = R.drawable.ic_backspace_28,
			label = stringResource(R.string.menu_option_clear_logs),
			tint = MaterialTheme.colors.red400,
			onclick = onDeleteClicked
		)
		Spacer(modifier = Modifier.padding(bottom = 8.dp))
		SecondaryButtonWide(
			label = stringResource(R.string.generic_cancel),
			onClicked = onCancel,
		)
		Spacer(modifier = Modifier.padding(bottom = 16.dp))
	}
}

@Composable
fun LogeMenuDeleteConfirm(
	onCancel: Callback,
	doRemoveKeyAndNavigateOut: Callback,
) {
	val coroutineScope = rememberCoroutineScope()
	val context = LocalContext.current
	BottomSheetConfirmDialog(
		title = stringResource(R.string.remove_key_confirm_title),
		message = stringResource(R.string.remove_key_confirm_text),
		ctaLabel = stringResource(R.string.remove_key_confirm_cta),
		onCancel = onCancel,
		onCta = {
			doRemoveKeyAndNavigateOut()
		}
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
private fun PreviewLogsMenu() {
	SignerNewTheme {
		LogsMenuGeneral(
			{},{},{},
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
private fun PreviewLogsMenuConfirm() {
	SignerNewTheme {
		LogeMenuDeleteConfirm(
			{},{},
		)
	}
}
