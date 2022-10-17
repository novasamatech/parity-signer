package io.parity.signer.screens.keysets

import android.content.res.Configuration
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.bottomsheets.MenuItemForBottomSheet
import io.parity.signer.components.base.SecondaryButtonBottomSheet
import io.parity.signer.models.EmptyNavigator
import io.parity.signer.models.Navigator
import io.parity.signer.ui.theme.SignerNewTheme


@Composable
fun KeySetMenuBottomSheet(
	navigator: Navigator,
) {
	val sidePadding = 24.dp
	Column(
		modifier = Modifier
			.fillMaxWidth()
			.padding(start = sidePadding, end = sidePadding, top = 8.dp),
	) {
		MenuItemForBottomSheet(
			iconId = R.drawable.ic_ios_share_28,
			label = stringResource(R.string.menu_option_export_private_key),
			tint = null,
			onclick = {
//				export all todo
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
private fun PreviewKeySetMenuBottomSheet() {
	SignerNewTheme {
		KeySetMenuBottomSheet(
			EmptyNavigator(),
		)
	}
}
