package io.parity.signer.screens.keysetdetails.filtermenu

import android.content.res.Configuration
import androidx.compose.foundation.layout.Column
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
import io.parity.signer.components.base.BottomSheetHeader
import io.parity.signer.components.base.RowButtonsBottomSheet
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.components.items.NetworkItemMultiselect
import io.parity.signer.domain.Callback
import io.parity.signer.domain.NetworkModel
import io.parity.signer.domain.NetworkState
import io.parity.signer.ui.theme.SignerNewTheme


//todo dmitry work in progress
@Composable
fun NetworkFilterMenu(
	networks: List<NetworkModel>,
	onConfirm: (List<NetworkModel>) -> Unit,
	onCancel: Callback,
) {
	val sidePadding = 24.dp
	Column(
		modifier = Modifier
			.fillMaxWidth()
			.padding(start = sidePadding, end = sidePadding, top = 8.dp),
	) {
		BottomSheetHeader(title = "Filter keys by network", //todo dmitry export
			onCloseClicked = onCancel)
		SignerDivider()
		networks.forEach {network ->
			NetworkItemMultiselect(network = network, //todo dmitry pass properly selected as this item handled elseware
				isSelected = false, onClick = {})
		}
		RowButtonsBottomSheet(
			labelCancel = stringResource(R.string.generic_clear_selection),
			labelCta = stringResource(id = R.string.generic_done),
			onClickedCancel = { onConfirm(emptyList()) },
			onClickedCta = { /*TODO*/ })
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
				onConfirm = {},
				onCancel = {},
		)
	}
}
