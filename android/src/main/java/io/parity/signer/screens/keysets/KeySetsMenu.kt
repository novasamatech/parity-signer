package io.parity.signer.screens.keysets

import android.content.res.Configuration
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import androidx.navigation.compose.rememberNavController
import io.parity.signer.R
import io.parity.signer.bottomsheets.MenuItemForBottomSheet
import io.parity.signer.components.base.SecondaryButtonBottomSheet
import io.parity.signer.ui.navigationselectors.KeySetsNavSubgraph
import io.parity.signer.ui.theme.SignerNewTheme


@Composable
fun KeySetsMenuBottomSheet(
	navigator: NavController,
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
				navigator.navigate(KeySetsNavSubgraph.export) {
					popUpTo(KeySetsNavSubgraph.home)
				}
			}
		)
		Spacer(modifier = Modifier.padding(bottom = 16.dp))
		SecondaryButtonBottomSheet(
			label = stringResource(R.string.generic_cancel),
		) {
			navigator.popBackStack()
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
		KeySetsMenuBottomSheet(
			rememberNavController(),
		)
	}
}
