package io.parity.signer.screens.logs

import android.content.res.Configuration
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.HdrPlus
import androidx.compose.material.icons.filled.PlusOne
import androidx.compose.material.icons.outlined.Circle
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.BottomSheetConfirmDialog
import io.parity.signer.components.base.SecondaryButtonBottomSheet
import io.parity.signer.models.AlertState
import io.parity.signer.models.Callback
import io.parity.signer.models.EmptyNavigator
import io.parity.signer.models.Navigator
import io.parity.signer.screens.keydetails.KeyDetailsDeleteConfirmBottomSheet
import io.parity.signer.screens.keydetails.MenuItemForBottomSheet
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.red400
import io.parity.signer.uniffi.Action


@Composable
fun LogsMenu(
	navigator: Navigator,
) {
	val state = remember {
		mutableStateOf(LogsState.GENERAL)
	}
	when (state.value) {
		LogsState.GENERAL -> LogsMenuGeneral(
			navigator = navigator,
			onDeleteClicked = { state.value = LogsState.DELETE_CONFIRM },
		)
		LogsState.DELETE_CONFIRM ->
			LogeMenuDeleteConfirm(
				onCancel = { state.value = LogsState.GENERAL },
				onRemoveKey = { navigator.navigate(Action.CLEAR_LOG) }
			)
	}
}

@Composable
private fun LogsMenuGeneral(
	navigator: Navigator,
	onDeleteClicked: Callback,
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
			onclick = { navigator.navigate(Action.CREATE_LOG_COMMENT) }
		)

		MenuItemForBottomSheet(
			iconId = R.drawable.ic_backspace_28,
			label = stringResource(R.string.menu_option_clear_logs),
			tint = MaterialTheme.colors.red400,
			onclick = onDeleteClicked
		)
		Spacer(modifier = Modifier.padding(bottom = 8.dp))
		SecondaryButtonBottomSheet(
			label = stringResource(R.string.generic_cancel),
		) {
			navigator.backAction()
		}
		Spacer(modifier = Modifier.padding(bottom = 16.dp))
	}
}

@Composable
fun LogeMenuDeleteConfirm(
	onCancel: Callback,
	onRemoveKey: Callback,
) {
	BottomSheetConfirmDialog(
		title = stringResource(R.string.remove_key_confirm_title),
		message = stringResource(R.string.remove_key_confirm_text),
		ctaLabel = stringResource(R.string.remove_key_confirm_cta),
		onCancel = onCancel,
		onCta = onRemoveKey,
	)
}

private enum class LogsState {
	GENERAL, DELETE_CONFIRM,
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
		LogsMenu(
			EmptyNavigator(),
		)
	}
}
