package io.parity.signer.screens.networks

import android.content.res.Configuration
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ChevronRight
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.components.networkicon.NetworkIcon
import io.parity.signer.components.panels.BottomBar2
import io.parity.signer.components.panels.BottomBar2State
import io.parity.signer.domain.*
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.textTertiary
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.MManageNetworks


@Composable
fun NetworksList(model: NetworksListModel, rootNavigator: Navigator) {
	Column(Modifier.background(MaterialTheme.colors.background)) {
		ScreenHeader(title = stringResource(R.string.networks_screen_title),
			onBack = { rootNavigator.backAction() })
		Column(
			Modifier
				.weight(1f)
				.verticalScroll(rememberScrollState())
		) {
			model.networks.forEach { network ->
				NetworkListItem(network) {
					rootNavigator.navigate(Action.GO_FORWARD, network.key)
				}
			}
		}
		BottomBar2(rootNavigator, BottomBar2State.SETTINGS)
	}
}

@Composable
private fun NetworkListItem(network: NetworkModel, callback: Callback) {
	Row(
		Modifier
			.padding(horizontal = 16.dp, vertical = 8.dp)
			.clickable(onClick = callback)
	) {
		NetworkIcon(networkLogoName = network.logo, size = 36.dp)
		Text(
			text = network.title,
			style = SignerTypeface.TitleS,
			color = MaterialTheme.colors.primary,
			modifier = Modifier
				.padding(start = 24.dp)
				.weight(1f)
		)
		Image(
			imageVector = Icons.Filled.ChevronRight,
			contentDescription = null,
			colorFilter = ColorFilter.tint(MaterialTheme.colors.textTertiary),
			modifier = Modifier.padding(start = 16.dp)
		)
	}
}


data class NetworksListModel(val networks: List<NetworkModel>)

fun MManageNetworks.toNetworksListModel(): NetworksListModel =
	NetworksListModel(
		networks = networks.map { it.toNetworkModel() }
	)


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
private fun PreviewNetworksList() {
	val model = NetworksListModel(
		listOf<NetworkModel>(
			NetworkModel.createStub(),
			NetworkModel.createStub().copy(title = "Kusama", logo = "kusama"),
			NetworkModel.createStub().copy(title = "Westend", logo = "westend"),
		)
	)
	SignerNewTheme {
		NetworksList(
			model,
			rootNavigator = EmptyNavigator(),
		)
	}
}
