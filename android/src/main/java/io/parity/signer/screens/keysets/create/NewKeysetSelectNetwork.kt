package io.parity.signer.screens.keysets.create

import SignerCheckbox
import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
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
import io.parity.signer.R
import io.parity.signer.components.base.NotificationFrameTextImportant
import io.parity.signer.components.base.PrimaryButtonWide
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.components.networkicon.NetworkIcon
import io.parity.signer.domain.Callback
import io.parity.signer.domain.KeySetModel
import io.parity.signer.domain.NetworkModel
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.fill6

//todo dmitry add this
@Composable
fun NewKeySetSelectNetwork(
	networks: List<NetworkModel>,
	selected: MutableState<Set<NetworkModel>>,
	onNetworkSelect: (NetworkModel) -> Unit, //todo dmitry
	onProceed: () -> Unit, // todo dmitry onProceed: (List<NetworkModel>) -> Unit,
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
			title = stringResource(R.string.new_key_set_backup_title),
			onBack = onBack,
		)
		Text(
			text = stringResource(R.string.new_key_set_backup_subtitle),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.BodyL,
			modifier = Modifier
                .padding(horizontal = 24.dp)
                .padding(bottom = 8.dp),
		)
		Column(
			modifier = Modifier
                .padding(horizontal = 8.dp)
                .background(
                    MaterialTheme.colors.fill6,
                    RoundedCornerShape(dimensionResource(id = R.dimen.plateDefaultCornerRadius))
                )
		) {
			networks.forEach { network ->
				NetworkItemMultiselect(
					network = network,
					isSelected = selected.value.contains(network)
				) { network ->
					onNetworkSelect(network)
				}
				SignerDivider()
			}
		}
		NotificationFrameTextImportant(
			message = stringResource(R.string.new_key_set_backup_warning_message),
			modifier = Modifier
				.padding(horizontal = 16.dp)
		)
		Spacer(modifier = Modifier.weight(1f))

		PrimaryButtonWide(
			label = stringResource(R.string.new_key_set_backup_cta),
			modifier = Modifier.padding(horizontal = 32.dp, vertical = 24.dp),
			onClicked = onProceed,
		)
	}
}


@Composable
private fun NetworkItemMultiselect(
	network: NetworkModel,
	isSelected: Boolean,
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
		SignerCheckbox(
			isChecked = isSelected,
			modifier = Modifier
				.padding(end = 8.dp)
		) {
			onClick(network)
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
	val selected = remember<MutableState<Set<NetworkModel>>> {
		mutableStateOf(setOf(networks[1]))
	}
	SignerNewTheme {
		NewKeySetSelectNetwork(networks, selected, {}, {}, {})
	}
}
