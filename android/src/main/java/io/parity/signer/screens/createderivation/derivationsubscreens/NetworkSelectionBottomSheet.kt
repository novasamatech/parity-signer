package io.parity.signer.screens.createderivation.derivationsubscreens

import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.outlined.Check
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.BottomSheetHeader
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.models.Callback
import io.parity.signer.models.NetworkModel
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.pink300


@Composable
fun NetworkSelectionBottomSheet(
	networks: List<NetworkModel>,
	currentlySelectedNetwork: NetworkModel,
	onClose: Callback,
	onSelect: (NetworkModel) -> Unit,
) {
	Column() {
		BottomSheetHeader(
			title = stringResource(R.string.network_selection_buttomsheet_title),
			onCloseClicked = onClose
		)
		SignerDivider()
		Column(
			modifier = Modifier
				.verticalScroll(rememberScrollState())
				.padding(horizontal = 24.dp)
		) {
			networks.forEach {
				NetworkElement(it, it == currentlySelectedNetwork) {
					onSelect(it)
				}
			}
		}
		SignerDivider()
	}
}

@Composable
private fun NetworkElement(
	network: NetworkModel,
	isSelected: Boolean,
	onClicked: Callback,
) {
	Row(
		modifier = Modifier
			.clickable(onClick = onClicked)
			.defaultMinSize(minHeight = 48.dp)
			.fillMaxWidth(1f),
		verticalAlignment = Alignment.CenterVertically,
	) {
		Text(
			text = network.title,
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleS,
			modifier = Modifier.weight(1f),
		)
		if (isSelected) {
			Image(
				imageVector = Icons.Outlined.Check,
				contentDescription = stringResource(R.string.network_selection_selected_network_mark),
				colorFilter = ColorFilter.tint(MaterialTheme.colors.pink300),
				modifier = Modifier
					.padding(horizontal = 8.dp)
					.size(28.dp)
					.align(Alignment.CenterVertically)
			)
		}
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
private fun PreviewNetworkSelectionBottomSheet() {
	SignerNewTheme {
		val model = listOf(
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
		NetworkSelectionBottomSheet(model, model[1], {}, { _ -> })
	}
}
