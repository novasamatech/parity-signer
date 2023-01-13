package io.parity.signer.screens.keyderivation.derivationsubscreens

import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ChevronRight
import androidx.compose.material.icons.filled.Help
import androidx.compose.material.icons.filled.HelpOutline
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.PrimaryButtonWide
import io.parity.signer.components.base.ScreenHeaderClose
import io.parity.signer.models.Callback
import io.parity.signer.models.NetworkModel
import io.parity.signer.screens.keyderivation.INITIAL_DERIVATION_PATH
import io.parity.signer.ui.theme.*

@Composable
fun DeriveKeyBaseScreen(
	selectedNetwork: NetworkModel,
	path: String,
	onClose: Callback,
	onNetworkSelectClicked: Callback,
	onDerivationPathHelpClicked: Callback,
	onDerivationMenuHelpClicked: Callback,
	onPathClicked: Callback,
	onCreateClicked: Callback,
	modifier: Modifier = Modifier
) {

	Column(modifier = modifier) {
		ScreenHeaderClose(
			title = stringResource(R.string.derivation_screen_title),
			onClose = onClose,
			onMenu = onDerivationMenuHelpClicked,
			differentMenuIcon = Icons.Filled.HelpOutline
		)

		Text(
			text = stringResource(R.string.derivation_screen_network_header),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleS,
			modifier = Modifier.padding(horizontal = 24.dp),
		)
		Row(
			modifier = Modifier
				.padding(horizontal = 24.dp)
				.padding(top = 14.dp, bottom = 8.dp)
				.fillMaxWidth(1f)
				.clickable(onClick = onNetworkSelectClicked)
				.background(
					MaterialTheme.colors.fill6,
					RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius))
				)
				.padding(top = 10.dp, bottom = 10.dp, start = 16.dp, end = 4.dp),
			verticalAlignment = Alignment.CenterVertically
		) {
			Text(
				text = stringResource(R.string.derivation_screen_network_label),
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.TitleS,
			)
			Spacer(modifier = Modifier.weight(1f))
			NetworkLabel(networkName = selectedNetwork.title)
			ChevronRight()
		}

		Row(
			modifier = Modifier.padding(horizontal = 24.dp, vertical = 14.dp)
		) {
			Text(
				text = stringResource(R.string.derivation_screen_derivation_header),
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.TitleS,
			)
			Image(
				imageVector = Icons.Filled.Help,
				contentDescription = stringResource(R.string.derivation_screen_dirivation_help_content_description),
				colorFilter = ColorFilter.tint(MaterialTheme.colors.pink300),
				modifier = Modifier
					.padding(horizontal = 8.dp)
					.clickable(onClick = onDerivationPathHelpClicked)
					.size(18.dp)
					.align(Alignment.CenterVertically)
			)
		}
		Row(
			modifier = Modifier
				.padding(horizontal = 24.dp)
				.fillMaxWidth(1f)
				.clickable(onClick = onPathClicked)
				.background(
					MaterialTheme.colors.fill6,
					RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius))
				)
				.padding(top = 10.dp, bottom = 10.dp, start = 16.dp, end = 4.dp),
			verticalAlignment = Alignment.CenterVertically
		) {
			Text(
				text = if (path == INITIAL_DERIVATION_PATH) {
					stringResource(R.string.derivation_screen_path_placeholder)
				} else {
					path
				}, //todo derivation hide password
				color = MaterialTheme.colors.textTertiary,
				style = SignerTypeface.TitleS,
			)
			Spacer(modifier = Modifier.weight(1f))
			ChevronRight()
		}
		Text(
			text = stringResource(R.string.derivation_screen_derivation_note_description),
			color = MaterialTheme.colors.textTertiary,
			style = SignerTypeface.CaptionM,
			modifier = Modifier.padding(horizontal = 24.dp, vertical = 12.dp),
		)
		Spacer(modifier = Modifier.weight(1f))
		PrimaryButtonWide(
			label = stringResource(R.string.derivation_screen_derivation_button_create),
			modifier = Modifier.padding(24.dp),
			isEnabled = false, //todo derivation
			onClicked = onCreateClicked,
		)
	}
}

@Composable
private fun NetworkLabel(networkName: String) {
	Box(
		modifier = Modifier
			.background(
				MaterialTheme.colors.fill12, RoundedCornerShape(12.dp)
			)
			.padding(horizontal = 8.dp, vertical = 2.dp),
		contentAlignment = Alignment.Center,
	) {
		Text(
			networkName,
			color = MaterialTheme.colors.textSecondary,
			style = SignerTypeface.BodyM,
		)
	}
}

@Composable
private fun ChevronRight() {
	Image(
		imageVector = Icons.Filled.ChevronRight,
		contentDescription = null,
		colorFilter = ColorFilter.tint(MaterialTheme.colors.textDisabled),
		modifier = Modifier.size(28.dp)
	)
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
private fun PreviewDeriveKeyBaseScreen() {
	SignerNewTheme {
		DeriveKeyBaseScreen(
			selectedNetwork = NetworkModel.createStub(),
			path = INITIAL_DERIVATION_PATH,
			onClose = {},
			onNetworkSelectClicked = {},
			onDerivationPathHelpClicked = {},
			onDerivationMenuHelpClicked = {},
			onPathClicked = {},
			onCreateClicked = {})
	}
}
