package io.parity.signer.components.items

import SignerCheckbox
import android.content.res.Configuration
import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.components.base.CheckIcon
import io.parity.signer.components.networkicon.NetworkIcon
import io.parity.signer.domain.NetworkModel
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface


@Composable
fun NetworkItemSelectable(
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
		AnimatedVisibility(
			visible = isSelected,
			enter = fadeIn(),
			exit = fadeOut(),
		) {
			CheckIcon(modifier = Modifier.padding(end = 16.dp))
		}
	}
}

@Composable
fun NetworkItemMultiselect(
	network: NetworkModel,
	isSelected: Boolean,
	modifier: Modifier = Modifier,
	onClick: (NetworkModel) -> Unit,
) {
	Row(
		modifier = modifier.clickable { onClick(network) },
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
			modifier = Modifier.padding(end = 8.dp),
			uncheckedColor = MaterialTheme.colors.primary,
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
private fun PreviewNetworkItem() {
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
	)
	SignerNewTheme {
		Column() {
			NetworkItemSelectable(
				network = networks[0],
				isSelected = true,
				onClick = {})
			NetworkItemSelectable(
				network = networks[0],
				isSelected = false,
				onClick = {})
			NetworkItemMultiselect(
				network = networks[0],
				isSelected = true,
				onClick = {}
			)
			NetworkItemMultiselect(
				network = networks[1],
				isSelected = false,
				onClick = {}
			)
		}
	}
}
