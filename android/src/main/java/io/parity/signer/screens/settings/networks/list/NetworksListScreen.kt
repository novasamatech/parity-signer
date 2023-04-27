package io.parity.signer.screens.settings.networks.list

import android.content.res.Configuration
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.ChevronRight
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.components.networkicon.NetworkIcon
import io.parity.signer.components.panels.BottomBar
import io.parity.signer.components.panels.BottomBarState
import io.parity.signer.components.panels.CameraParentScreen
import io.parity.signer.components.panels.CameraParentSingleton
import io.parity.signer.domain.*
import io.parity.signer.screens.createderivation.derivationsubscreens.NetworkHelpAlarm
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.fill12
import io.parity.signer.ui.theme.textTertiary
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.MManageNetworks


@Composable
fun NetworksListScreen(
	model: NetworksListModel,
	rootNavigator: Navigator,
	onNetworkHelp: Callback,
	onAddNetwork: Callback,
) {
	Column(Modifier.background(MaterialTheme.colors.background)) {
		ScreenHeader(
			title = stringResource(R.string.networks_screen_title),
			onBack = { rootNavigator.backAction() }
		)
		Spacer(modifier = Modifier.padding(top = 10.dp))
		Column(
			Modifier
				.weight(1f)
				.verticalScroll(rememberScrollState())
		) {
			model.networks.forEach { network ->
				NetworkListItem(network) {
					rootNavigator.navigate(Action.GO_FORWARD, network.key)
					CameraParentSingleton.lastPossibleParent =
						CameraParentScreen.NetworkDetailsScreen(network.key)
				}
			}
			AddNetworkItem(onAddNetwork)
			NetworkHelpAlarm(
				Modifier
					.padding(horizontal = 8.dp, vertical = 16.dp)
					.clickable(onClick = onNetworkHelp)
			)
		}
		BottomBar(
			rootNavigator, BottomBarState.SETTINGS,
			skipRememberCameraParent = true
		) {
			rootNavigator.backAction()
		}
		LaunchedEffect(key1 = Unit) {
			CameraParentSingleton.lastPossibleParent =
				CameraParentScreen.NetworkListScreen
		}
	}
}

@Composable
private fun AddNetworkItem(callback: Callback) {
	Row(
		Modifier
			.padding(horizontal = 16.dp, vertical = 8.dp)
			.clickable(onClick = callback),
		verticalAlignment = Alignment.CenterVertically,
	) {
		Box(
			modifier = Modifier
				.size(36.dp)
				.background(MaterialTheme.colors.fill12, CircleShape),
			contentAlignment = Alignment.Center
		) {
			Image(
				imageVector = Icons.Default.Add,
				contentDescription = stringResource(R.string.networks_screen_add_net_network),
				colorFilter = ColorFilter.tint(MaterialTheme.colors.textTertiary),
				modifier = Modifier
					.size(24.dp)
			)
		}
		Text(
			text = stringResource(R.string.networks_screen_add_net_network),
			style = SignerTypeface.TitleS,
			color = MaterialTheme.colors.primary,
			modifier = Modifier
				.padding(start = 12.dp)
				.weight(1f)
		)
	}
}


@Composable
private fun NetworkListItem(network: NetworkModel, callback: Callback) {
	Row(
		Modifier
			.padding(horizontal = 16.dp, vertical = 8.dp)
			.clickable(onClick = callback),
		verticalAlignment = Alignment.CenterVertically,
	) {
		NetworkIcon(networkLogoName = network.logo, size = 36.dp)
		Text(
			text = network.title,
			style = SignerTypeface.TitleS,
			color = MaterialTheme.colors.primary,
			modifier = Modifier
				.padding(start = 12.dp)
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
		NetworksListScreen(
			model,
			rootNavigator = EmptyNavigator(),
			{},
			{},
		)
	}
}
