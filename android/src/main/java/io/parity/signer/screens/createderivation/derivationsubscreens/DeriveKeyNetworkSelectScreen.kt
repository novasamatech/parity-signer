package io.parity.signer.screens.createderivation.derivationsubscreens

import android.content.res.Configuration
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ChevronRight
import androidx.compose.material.icons.outlined.HelpOutline
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.SpanStyle
import androidx.compose.ui.text.buildAnnotatedString
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.ScreenHeaderClose
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.components.networkicon.NetworkIcon
import io.parity.signer.domain.Callback
import io.parity.signer.domain.NetworkModel
import io.parity.signer.screens.createderivation.SelectedNetwork
import io.parity.signer.ui.theme.*

@Composable
fun DeriveKeyNetworkSelectScreen(
	networks: List<NetworkModel>,
	onClose: Callback,
	onNetworkSelect: (SelectedNetwork) -> Unit,
	onNetworkHelp: Callback,
	modifier: Modifier = Modifier
) {

	Column(modifier) {
		ScreenHeaderClose(
			title = stringResource(R.string.derivation_network_select_title),
			onClose = onClose,
		)
		Column(
			modifier = Modifier
				.verticalScroll(rememberScrollState())
				.padding(horizontal = 8.dp)
				.background(
					MaterialTheme.colors.fill6,
					RoundedCornerShape(dimensionResource(id = R.dimen.plateDefaultCornerRadius))
				)
		) {
			networks.forEach { network ->
				NetworkItem(network) { network ->
					onNetworkSelect(SelectedNetwork.Network(network))
				}
				SignerDivider()
			}
			AllNetworksItem() {
				onNetworkSelect(SelectedNetwork.AllNetworks)
			}
		}
		NetworkAlarm(
			Modifier
				.padding(horizontal = 8.dp)
				.clickable(onClick = onNetworkHelp))
		Spacer(modifier = Modifier.weight(1f))
	}
}

@Composable
private fun NetworkItem(
	network: NetworkModel,
	onClick: (NetworkModel) -> Unit,
) {
	Row(
		modifier = Modifier.clickable { onClick(network) },
		verticalAlignment = Alignment.CenterVertically
	) {
		NetworkIcon(
			networkLogoName = network.logo,
			modifier = Modifier
				.padding(
					top = 16.dp,
					bottom = 16.dp,
					start = 16.dp,
					end = 12.dp
				)
				.size(36.dp),
		)
		Text(
			text = network.title,
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleS,
		)
		Spacer(modifier = Modifier.weight(1f))
		Image(
			imageVector = Icons.Filled.ChevronRight,
			contentDescription = null,
			colorFilter = ColorFilter.tint(MaterialTheme.colors.textTertiary),
			modifier = Modifier
                .padding(2.dp)// because it's 28 not 32pd
                .padding(end = 16.dp)
                .size(28.dp)
		)
	}
}

@Composable
private fun AllNetworksItem(
	onClick: (SelectedNetwork) -> Unit,
) {
	Row(
		modifier = Modifier
            .height(70.dp)
            .clickable { onClick(SelectedNetwork.AllNetworks) },
		verticalAlignment = Alignment.CenterVertically
	) {
		Text(
			text = stringResource(R.string.derive_key_network_all),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleS,
			modifier = Modifier
                .padding(horizontal = 16.dp)
                .weight(1f)
		)
		Image(
			imageVector = Icons.Filled.ChevronRight,
			contentDescription = null,
			colorFilter = ColorFilter.tint(MaterialTheme.colors.textTertiary),
			modifier = Modifier
                .padding(2.dp)// because it's 28 not 32pd
                .padding(end = 16.dp)
                .size(28.dp)
		)
	}
}

@Composable
private fun NetworkAlarm(modifier: Modifier = Modifier) {
	val innerShape =
		RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius))
	Row(
		modifier = modifier
            .padding(vertical = 8.dp)
            .border(
                BorderStroke(1.dp, MaterialTheme.colors.appliedStroke),
                innerShape
            )

	) {

		Text(
			text = stringResource(R.string.derivation_create_help_network_setup_label),
			color = MaterialTheme.colors.textTertiary,
			style = SignerTypeface.BodyM,
			modifier = Modifier
                .weight(1f)
                .padding(start = 16.dp, top = 16.dp, bottom = 16.dp)
		)
		Icon(
			imageVector = Icons.Outlined.HelpOutline,
			contentDescription = null,
			tint = MaterialTheme.colors.pink300,
			modifier = Modifier
                .align(Alignment.CenterVertically)
                .padding(start = 18.dp, end = 18.dp)
		)
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
private fun PreviewDeriveKeyNetworkSelectScreen() {
	val networks = listOf(
		NetworkModel(
			key = "0",
			logo = "polkadot",
			title = "Polkadot",
		),
		NetworkModel(
			key = "1",
			logo = "Kusama",
			title = "Kusama",
		),
		NetworkModel(
			key = "2",
			logo = "Wastend",
			title = "Wastend",
		),
	)
	SignerNewTheme {
		DeriveKeyNetworkSelectScreen(
			networks = networks,
			onClose = {},
			onNetworkSelect = {},
			onNetworkHelp = {},
		)
	}
}
