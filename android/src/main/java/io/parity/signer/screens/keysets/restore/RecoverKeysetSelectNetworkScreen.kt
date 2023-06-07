package io.parity.signer.screens.keysets.restore

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.MutableState
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.lifecycle.viewmodel.compose.viewModel
import io.parity.signer.R
import io.parity.signer.components.base.NotificationFrameText
import io.parity.signer.components.base.PrimaryButtonWide
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.domain.Callback
import io.parity.signer.domain.Navigator
import io.parity.signer.domain.NetworkModel
import io.parity.signer.screens.keysets.create.backupstepscreens.NetworkItemMultiselect
import io.parity.signer.screens.keysets.create.backupstepscreens.NetworkItemMultiselectAll
import io.parity.signer.screens.keysets.create.backupstepscreens.NewKeySetNetworksViewModel
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.fill6


@Composable
fun RecoverKeysetSelectNetworkScreen(
	seedName: String,
	seedPhrase: String,
	rootNavigator: Navigator,
	onBack: Callback,
	modifier: Modifier = Modifier,
) {
	val networksViewModel: NewKeySetNetworksViewModel = viewModel()
	val selected: MutableState<Set<String>> =
		remember {
			mutableStateOf(
				networksViewModel.getDefaultPreselectedNetworks()
					.map { it.key }.toSet()
			)
		}
	val networks = networksViewModel.getAllNetworks()

	Box(modifier = modifier) {
		RecoverKeysetSelectNetworkScreenPrivate(
			networks = networks,
			selectedNetworkKeys = selected.value,
			onNetworkClick = { network ->
				selected.value = if (selected.value.contains(network.key)) {
					selected.value - network.key
				} else {
					selected.value + network.key
				}
			},
			onProceed = {
				networksViewModel.createKeySetWithNetworks(
					seedName = seedName, seedPhrase = seedPhrase,
					networksForKeys = selected.value.mapNotNull { selected -> networks.find { it.key == selected } }
						.toSet(),
					navigator = rootNavigator,
				)
			},
			onAddAll = {
				selected.value = if (selected.value.size == networks.size) {
					networksViewModel.getDefaultPreselectedNetworks().map { it.key }
						.toSet()
				} else {
					networks.map { it.key }.toSet()
				}
			},
			onBack = onBack,
		)
	}
}

@Composable
private fun RecoverKeysetSelectNetworkScreenPrivate(
	networks: List<NetworkModel>,
	selectedNetworkKeys: Set<String>,
	onNetworkClick: (NetworkModel) -> Unit,
	onProceed: Callback,
	onAddAll: Callback,
	onBack: Callback
) {
	Column(
		modifier = Modifier
			.fillMaxSize(1f)
			.background(MaterialTheme.colors.background)
			.verticalScroll(rememberScrollState()),
		horizontalAlignment = Alignment.CenterHorizontally,
	) {
		ScreenHeader(
			title = stringResource(R.string.keyset_recover_keys_title),
			onBack = onBack,
		)
		Text(
			text = stringResource(R.string.keyset_recover_keys_subtitle),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.BodyL,
			modifier = Modifier
				.padding(horizontal = 24.dp)
		)
		Column(
			modifier = Modifier
				.padding(horizontal = 8.dp, vertical = 16.dp)
				.background(
					MaterialTheme.colors.fill6,
					RoundedCornerShape(dimensionResource(id = R.dimen.plateDefaultCornerRadius))
				)
		) {
			networks.forEach { network ->
				NetworkItemMultiselect(
					network = network,
					isSelected = selectedNetworkKeys.contains(network.key)
				) { network ->
					onNetworkClick(network)
				}
				SignerDivider()
			}
			NetworkItemMultiselectAll(onAddAll)
		}
		NotificationFrameText(
			message = stringResource(R.string.keyset_recover_keys_notification_text),
			modifier = Modifier
				.padding(horizontal = 16.dp)
		)
		Spacer(modifier = Modifier.weight(1f))

		PrimaryButtonWide(
			label = stringResource(R.string.keyset_recover_keys_cta),
			modifier = Modifier.padding(horizontal = 32.dp, vertical = 24.dp),
			onClicked = onProceed,
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
private fun PreviewNewKeySetSelectNetwork() {
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
	val selected = setOf(networks[1].key)
	SignerNewTheme {
		RecoverKeysetSelectNetworkScreenPrivate(networks, selected, {}, {}, {}, {})
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
private fun PreviewNewKeySetSelectNetworkVeryLong() {
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
	val selected = setOf(networks[1].key)
	SignerNewTheme {
		RecoverKeysetSelectNetworkScreenPrivate(networks, selected, {}, {}, {}, {})
	}
}
