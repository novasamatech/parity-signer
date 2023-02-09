package io.parity.signer.screens.keysetdetails

import android.content.res.Configuration
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
import io.parity.signer.components.base.SecondaryButtonWide
import io.parity.signer.domain.Callback
import io.parity.signer.domain.EmptyNavigator
import io.parity.signer.domain.Navigator
import io.parity.signer.domain.NetworkState
import io.parity.signer.screens.keydetails.MenuItemForBottomSheet
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.red400
import io.parity.signer.uniffi.Action

@Composable
fun KeySetDetailsMenu(
	navigator: Navigator,
	networkState: State<NetworkState?>,
	removeSeed: Callback,
	onSelectKeysClicked: Callback,
	onBackupClicked: Callback,
	onCancel: Callback,
) {
	val state = remember {
		mutableStateOf(KeySetDetailsMenuState.GENERAL)
	}
	when (state.value) {
		KeySetDetailsMenuState.GENERAL ->
			KeyDetailsMenuGeneral(
				navigator = navigator,
				networkState = networkState,
				onDeleteClicked = {
					state.value = KeySetDetailsMenuState.DELETE_CONFIRM
				},
				onSelectKeysClicked = onSelectKeysClicked,
				onBackupClicked = onBackupClicked,
				onCancel = onCancel,
			)
		KeySetDetailsMenuState.DELETE_CONFIRM ->
			KeySetDeleteConfirmBottomSheet(
				onCancel = { state.value = KeySetDetailsMenuState.GENERAL },
				onRemoveKey = removeSeed,
			)
	}
}

//todo dmitry switch state back to general if sheet is closed
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
	networkState: State<NetworkState?>,
	onSelectKeysClicked: Callback,
	onBackupClicked: Callback,
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
			Icons.Outlined.Circle,
			label = stringResource(R.string.menu_option_select_key),
			onclick = onSelectKeysClicked
		)

		MenuItemForBottomSheet(
			iconId = R.drawable.ic_library_add_28,
			label = stringResource(R.string.menu_option_derive_from_key),
			onclick = {
				if (networkState.value == NetworkState.None)
					navigator.navigate(Action.NEW_KEY)
				else
					navigator.navigate(Action.SHIELD)
			}
		)

		MenuItemForBottomSheet(
			iconId = R.drawable.ic_settings_backup_restore_28,
			label = stringResource(R.string.menu_option_backup_key_set),
			onclick = {
				if (networkState.value == NetworkState.None)
					onBackupClicked()
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
		SecondaryButtonWide(
			label = stringResource(R.string.generic_cancel),
			onClicked = onCancel
		)
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
		val state = remember { mutableStateOf(NetworkState.None) }
		KeySetDetailsMenu(
			EmptyNavigator(), state, {}, {}, {}, {},
		)
	}
}
