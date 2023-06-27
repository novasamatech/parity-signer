package io.parity.signer.screens.scan.transaction.dynamicderivations

import android.content.res.Configuration
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
import io.parity.signer.domain.Callback
import io.parity.signer.domain.KeySetModel
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface

@Composable
fun AddDerivedKeysScreen(
	onBack: Callback
) {
	Column(
		modifier = Modifier
			.verticalScroll(rememberScrollState()),
	) {
		ScreenHeader(
			onBack = onBack,
			title = null,
			modifier = Modifier.padding(horizontal = 8.dp)
		)
		Text(
			text = "Add Derived Keys",
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleL,
			modifier = Modifier.padding(horizontal = 24.dp),
		)
		Text(
			text = "Сheck the keys and scan QR code into Omni Wallet app",
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.BodyL,
			modifier = Modifier
				.padding(horizontal = 24.dp)
				.padding(top = 8.dp, bottom = 20.dp),
		)

		//todo dmitry list of keysets

		Text(
			text = "Scan QR code to add the keys",//todo dmitry export text in this screen
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.BodyL,
			modifier = Modifier
				.padding(horizontal = 24.dp)
				.padding(top = 8.dp, bottom = 20.dp),
		)
		//todo dmitry qr code


	}
}


data class AddDerivedKeysModel(val keysets: List<KeySetModel>)


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
private fun PreviewAddDerivedKeysScreen() {


	SignerNewTheme {
		AddDerivedKeysScreen(onBack = {})
	}
}
