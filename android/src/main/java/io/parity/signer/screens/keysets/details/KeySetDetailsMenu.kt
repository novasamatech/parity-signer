package io.parity.signer.screens.keysets.details

import android.content.res.Configuration
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.outlined.Circle
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import androidx.navigation.compose.rememberNavController
import io.parity.signer.R
import io.parity.signer.components.base.SecondaryButtonBottomSheet
import io.parity.signer.models.EmptyNavigator
import io.parity.signer.models.Navigator
import io.parity.signer.screens.keydetails.MenuItemForBottomSheet
import io.parity.signer.ui.navigationselectors.KeySetsNavSubgraph
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.red400

//todo dmitry find existing file
@Composable
fun KeyDetailsMenu(
	navigator: Navigator,
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
			onclick = {
				//todo dmitry
			}
		)


		MenuItemForBottomSheet(
			iconId = R.drawable.ic_library_add_28,
			label = stringResource(R.string.menu_option_derive_from_key),
			onclick = {
//				state.value = KeyDetailsMenuState.PRIVATE_KEY_CONFIRM
			}
		)

		MenuItemForBottomSheet(
			iconId = R.drawable.ic_settings_backup_restore_28,
			label = stringResource(R.string.menu_option_backup_key_set),
			onclick = {
				//
			}
		)

		MenuItemForBottomSheet(
			iconId = R.drawable.ic_backspace_28,
			label = stringResource(R.string.menu_option_forget_delete_key),
			tint = MaterialTheme.colors.red400,
			onclick = {
//				state.value = KeyDetailsMenuState.DELETE_CONFIRM
			}
		)
		Spacer(modifier = Modifier.padding(bottom = 16.dp))
		SecondaryButtonBottomSheet(
			label = stringResource(R.string.generic_cancel),
		) {
			navigator.backAction()
		}
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
private fun PreviewKeyDetailsMenu() {
	SignerNewTheme {
		KeyDetailsMenu(
			EmptyNavigator(),
		)
	}
}
