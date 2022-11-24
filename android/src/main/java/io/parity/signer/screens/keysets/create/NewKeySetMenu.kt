package io.parity.signer.screens.keysets.create

import android.content.res.Configuration
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.BottomSheetHeader
import io.parity.signer.components.base.SecondaryButtonBottomSheet
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.models.AlertState
import io.parity.signer.models.EmptyNavigator
import io.parity.signer.models.Navigator
import io.parity.signer.screens.keydetails.MenuItemForBottomSheet
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.uniffi.Action

@Composable
fun NewSeedMenu(
	alertState: State<AlertState?>,
	navigator: Navigator,
) {
	val sidePadding = 24.dp
	Column() {
		BottomSheetHeader(
			title = stringResource(R.string.add_key_set_menu_title),
			modifier = Modifier.padding(vertical = 16.dp),
			onCloseClicked = null
		)
		SignerDivider(padding = 24.dp)

		Column(
			modifier = Modifier
				.fillMaxWidth()
				.padding(start = sidePadding, end = sidePadding, top = 8.dp),
		) {
			MenuItemForBottomSheet(
				iconId = R.drawable.ic_add_28,
				label = stringResource(R.string.add_key_set_menu_add),
				onclick = {
					if (alertState.value == AlertState.None)
						navigator.navigate(Action.NEW_SEED)
					else
						navigator.navigate(Action.SHIELD)
				}
			)

			MenuItemForBottomSheet(
				iconId = R.drawable.ic_download_offline_28,
				label = stringResource(R.string.add_key_set_menu_recover),
				onclick = {
					if (alertState.value == AlertState.None)
						navigator.navigate(Action.RECOVER_SEED)
					else
						navigator.navigate(Action.SHIELD)
				}
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
private fun PreviewNewSeedMenu() {
	val state = remember { mutableStateOf(AlertState.Past) }
	SignerNewTheme {
		NewSeedMenu(
			state,
			EmptyNavigator(),
		)
	}
}
