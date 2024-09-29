package io.parity.signer.screens.keysetdetails.filtermenu

import android.content.res.Configuration
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.MutableState
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.BottomSheetHeader
import io.parity.signer.components.base.RowButtonsBottomSheet
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.components.items.NetworkItemMultiselect
import io.parity.signer.domain.Callback
import io.parity.signer.domain.NetworkModel
import io.parity.signer.ui.theme.SignerNewTheme


@Composable
fun NetworkFilterMenu(
	networks: List<NetworkModel>,
	initialSelection: Set<String>,
	onConfirm: (Set<NetworkModel>) -> Unit,
	onCancel: Callback,
) {

	val selected: MutableState<Set<NetworkModel>> =
		remember {
			mutableStateOf(
				initialSelection.mapNotNull { selected -> networks.firstOrNull { it.key == selected } }
					.toSet()
			)
		}

	NetworkFilterMenu(
		networks = networks,
		selectedNetworks = selected.value,
		onClick = { network ->
			if (selected.value.contains(network)) {
				selected.value = selected.value - network
			} else {
				selected.value = selected.value + network
			}
		},
		onConfirm = { onConfirm(selected.value) },
		onClean = { onConfirm(emptySet()) },
		onCancel = onCancel,
	)
}


@Composable
private fun NetworkFilterMenu(
	networks: List<NetworkModel>,
	selectedNetworks: Set<NetworkModel>,
	onClick: (NetworkModel) -> Unit,
	onConfirm: Callback,
	onClean: Callback,
	onCancel: Callback,
) {
	Column(
		modifier = Modifier
			.fillMaxWidth(),
	) {
		BottomSheetHeader(
			title = stringResource(R.string.network_filters_header),
			onClose = onCancel
		)
		SignerDivider(sidePadding = 24.dp)
		networks.forEach { network ->
			NetworkItemMultiselect(
				modifier = Modifier.padding(start = 8.dp, end = 4.dp),
				network = network,
				isSelected = selectedNetworks.contains(network),
				onClick = onClick
			)
		}
		RowButtonsBottomSheet(
			modifier = Modifier
				.padding(24.dp)
				.padding(top = 8.dp),
			isCtaEnabled = selectedNetworks.isNotEmpty(),
			labelCancel = stringResource(R.string.generic_clear_selection),
			labelCta = stringResource(id = R.string.generic_done),
			onClickedCancel = onClean,
			onClickedCta = onConfirm,
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
private fun PreviewNetworkFilterMenu() {
	val networks = listOf(
		NetworkModel(
			key = "0",
			logo = "polkadot",
			title = "Polkadot",
			pathId = "polkadot",
		),
		NetworkModel(
			key = "1",
			logo = "Kusama",
			title = "Kusama",
			pathId = "polkadot",
		),
		NetworkModel(
			key = "2",
			logo = "Wastend",
			title = "Wastend",
			pathId = "polkadot",
		),
	)
	SignerNewTheme {
		NetworkFilterMenu(
			networks = networks,
			selectedNetworks = networks.subList(1, 1).toSet(),
			onClick = {},
			onConfirm = {},
			onClean = {},
			onCancel = {},
		)
	}
}
