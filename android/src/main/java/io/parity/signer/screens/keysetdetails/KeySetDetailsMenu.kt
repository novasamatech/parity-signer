package io.parity.signer.screens.keysetdetails

import android.content.res.Configuration
import android.telecom.Call
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.icons.Icons
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
fun KeySetDetailsMenu(
	navigator: Navigator,
	alertState: State<AlertState?>,
	removeSeed: Callback,
	onSelectKeysClicked: Callback,
) {
	val state = remember {
		mutableStateOf(KeySetDetailsMenuState.GENERAL)
	}
	when (state.value) {
		KeySetDetailsMenuState.GENERAL -> KeyDetailsMenuGeneral(
			navigator = navigator,
			alertState = alertState,
			onDeleteClicked = { state.value = KeySetDetailsMenuState.DELETE_CONFIRM },
			onSelectKeysClicked = onSelectKeysClicked,
		)
		KeySetDetailsMenuState.DELETE_CONFIRM ->
			KeySetDeleteConfirmBottomSheet(
				onCancel = { state.value = KeySetDetailsMenuState.GENERAL },
				onRemoveKey = removeSeed,
			)
	}
}

@Composable
fun KeySetDeleteConfirmBottomSheet(
	onCancel: Callback,
	onRemoveKey: Callback,
) {
	BottomSheetConfirmDialog(
		title = stringResource(R.string.remove_key_set_confirm_title),
		message = stringResource(R.string.remove_key_set_confirm_text),
		ctaLabel = stringResource(R.string.remove_key_set_confirm_cta),
		onCancel = onCancel,
		onCta = onRemoveKey,
	)
}

@Composable
fun KeyDetailsMenuGeneral(
	navigator: Navigator,
	alertState: State<AlertState?>,
	onSelectKeysClicked: Callback,
	onDeleteClicked: Callback,
) {
	val sidePadding = 24.dp
	Column(
		modifier = Modifier
			.fillMaxWidth()
			.padding(start = sidePadding, end = sidePadding, top = 8.dp),
	) {

		MenuItemForBottomSheet(
			Icons.Outlined.Circle,
			label = stringResource(R.string.menu_option_select_key),
			onclick = onSelectKeysClicked
		)

		MenuItemForBottomSheet(
			iconId = R.drawable.ic_library_add_28,
			label = stringResource(R.string.menu_option_derive_from_key),
			onclick = {
				if (alertState.value == AlertState.None)
					navigator.navigate(Action.NEW_KEY)
				else
					navigator.navigate(Action.SHIELD)
			}
		)

		MenuItemForBottomSheet(
			iconId = R.drawable.ic_settings_backup_restore_28,
			label = stringResource(R.string.menu_option_backup_key_set),
			onclick = {
				if (alertState.value == AlertState.None)
					navigator.navigate(Action.BACKUP_SEED)
				else
					navigator.navigate(Action.SHIELD)
			}
		)

		MenuItemForBottomSheet(
			iconId = R.drawable.ic_backspace_28,
			label = stringResource(R.string.menu_option_forget_delete_key),
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


private enum class KeySetDetailsMenuState {
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
private fun PreviewKeyDetailsMenu() {
	SignerNewTheme {
		val state = remember { mutableStateOf(AlertState.None) }
		KeySetDetailsMenu(
			EmptyNavigator(), state, {}, {},
		)
	}
}
