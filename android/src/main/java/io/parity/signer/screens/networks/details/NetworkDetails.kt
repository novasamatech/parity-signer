package io.parity.signer.screens.networks.details

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.components.networkicon.NetworkIcon
import io.parity.signer.domain.EmptyNavigator
import io.parity.signer.domain.Navigator
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface

@Composable
fun NetworkDetailsScreen(
	model: NetworkDetailsModel,
	rootNavigator: Navigator
) {
	Column(Modifier.background(MaterialTheme.colors.background)) {

		ScreenHeader(title = null, onBack = { rootNavigator.backAction() },
			onMenu = {
				//todo dmitry implement
			})
		Column(
			Modifier
				.weight(1f)
				.verticalScroll(rememberScrollState())
		) {

		NetworkIcon(networkLogoName = model.logo, size = 56.dp)
		}
		Text(
			text = model.title,
			style = SignerTypeface.TitleM,
			color = MaterialTheme.colors.primary,
			modifier = Modifier
				.padding(start = 24.dp)
				.weight(1f)
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
private fun PreviewNetworkDetailsScreen() {
	val model = NetworkDetailsModel.createStub()
	SignerNewTheme {
		NetworkDetailsScreen(
			model,
			rootNavigator = EmptyNavigator(),
		)
	}
}

