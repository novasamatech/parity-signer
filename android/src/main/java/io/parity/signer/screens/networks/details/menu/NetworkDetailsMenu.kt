package io.parity.signer.screens.networks.details.menu

import android.content.res.Configuration
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.outlined.SystemUpdateAlt
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
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
fun KeyDetailsMenuGeneral(
	navigator: Navigator,
	networkState: State<NetworkState?>,
	onSignNetworkSpecs: Callback,
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
			Icons.Outlined.SystemUpdateAlt,
			label = stringResource(R.string.network_details_menu_sign_specs),
			onclick = {
				if (networkState.value == NetworkState.None)
					onSignNetworkSpecs()
				else
					navigator.navigate(Action.SHIELD)
			}
		)

		MenuItemForBottomSheet(
			iconId = R.drawable.ic_backspace_28,
			label = stringResource(R.string.network_details_menu_delete_network),
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
private fun PreviewKeyDetailsMenuGeneral() {
	SignerNewTheme {
		val state = remember { mutableStateOf(NetworkState.None) }
		KeyDetailsMenuGeneral(
			EmptyNavigator(), state, {}, {}, {},
		)
	}
}
