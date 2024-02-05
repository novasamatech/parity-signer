package io.parity.signer.screens.createderivation.derivationsubscreens

import android.content.res.Configuration
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.outlined.HelpOutline
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.PrimaryButtonWide
import io.parity.signer.components.base.ScreenHeaderWithButton
import io.parity.signer.components.base.SecondaryButtonWide
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.components.items.NetworkItemSelectable
import io.parity.signer.domain.Callback
import io.parity.signer.domain.NetworkModel
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.appliedStroke
import io.parity.signer.ui.theme.fill6
import io.parity.signer.ui.theme.pink300
import io.parity.signer.ui.theme.textTertiary

@Composable
fun DeriveKeyNetworkSelectScreen(
	networks: List<NetworkModel>,
	preselectd: NetworkModel?,
	onClose: Callback,
	onAdvancePath: (NetworkModel) -> Unit,
	onFastCreate: (NetworkModel) -> Unit,
	onNetworkHelp: Callback,
	modifier: Modifier = Modifier
) {
	val selected = remember { mutableStateOf(preselectd) }

	Column(modifier) {
		ScreenHeaderWithButton(
			canProceed = false,
			title = stringResource(R.string.create_derivation_title),
			btnText = null,
			backNotClose = false,
			onClose = onClose,
			onDone = null,
		)
		Column(
			modifier = Modifier
				.fillMaxHeight(1f)
				.verticalScroll(rememberScrollState())
		) {
			Text(
				text = stringResource(R.string.derivation_network_select_title),
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.BodyL,
				modifier = Modifier
					.padding(horizontal = 16.dp, vertical = 8.dp)
			)
			Column(
				modifier = Modifier
					.padding(horizontal = 8.dp, vertical = 8.dp)
					.background(
						MaterialTheme.colors.fill6,
						RoundedCornerShape(dimensionResource(id = R.dimen.plateDefaultCornerRadius))
					)
			) {
				networks.forEach { network ->
					NetworkItemSelectable(
						network,
						isSelected = selected.value == network,
					) { network ->
						selected.value = network
					}
					SignerDivider()
				}
			}
			NetworkHelpAlarm(
				Modifier
					.padding(horizontal = 24.dp)
					.clickable(onClick = onNetworkHelp)
			)
			Spacer(modifier = Modifier.weight(1f))
			val network = selected.value
			PrimaryButtonWide(
				label = stringResource(R.string.derivation_create_fast_cta),
				modifier = Modifier.padding(horizontal = 24.dp)
					.padding(top = 24.dp),
				isEnabled = network != null,
				onClicked = {
					if (network != null) {
						onFastCreate(network)
					}
				},
			)
			SecondaryButtonWide(
				label = stringResource(R.string.derivation_create_advanced_cta),
				modifier = Modifier.padding(24.dp),
				isEnabled = network != null,
				onClicked = {
					if (network != null) {
						onAdvancePath(network)
					}
				},
			)
		}
	}
}

@Composable
fun NetworkHelpAlarm(modifier: Modifier = Modifier) {
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
			pathId = "polkadot",
		),
		NetworkModel(
			key = "1",
			logo = "Kusama",
			title = "Kusama",
			pathId = "kusama",
		),
		NetworkModel(
			key = "2",
			logo = "Wastend",
			title = "Wastend",
			pathId = "wastend",
		),
	)
	SignerNewTheme {
		DeriveKeyNetworkSelectScreen(
			networks = networks,
			preselectd = networks[1],
			onClose = {},
			onAdvancePath = {},
			onFastCreate = {},
			onNetworkHelp = {},
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
private fun PreviewDeriveKeyNetworkMenu() {
	val networks = mutableListOf<NetworkModel>()
	repeat(10) {
		networks.add(
			NetworkModel(
				key = "$it",
				logo = "polkadot",
				title = "Polkadot$it",
				pathId = "polkadot$it",
			)
		)
	}
	SignerNewTheme {
		DeriveKeyNetworkSelectScreen(
			networks = networks,
			preselectd = networks[1],
			onClose = {},
			onAdvancePath = {},
			onFastCreate = {},
			onNetworkHelp = {},
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
private fun PreviewDeriveKeyNetworkNoSelection() {
	val networks = mutableListOf<NetworkModel>()
	repeat(2) {
		networks.add(
			NetworkModel(
				key = "$it",
				logo = "polkadot",
				title = "Polkadot$it",
				pathId = "polkadot$it",
			)
		)
	}
	SignerNewTheme {
		DeriveKeyNetworkSelectScreen(
			networks = networks,
			preselectd = null,
			onClose = {},
			onAdvancePath = {},
			onFastCreate = {},
			onNetworkHelp = {},
		)
	}
}
