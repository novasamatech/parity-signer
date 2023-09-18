package io.parity.signer.screens.settings.networks.signspecs.view

import android.content.res.Configuration
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.ScreenHeaderClose
import io.parity.signer.components.sharedcomponents.KeyCardSignature
import io.parity.signer.domain.Callback
import io.parity.signer.screens.settings.networks.signspecs.SignSpecsListModel
import io.parity.signer.ui.theme.SignerNewTheme

@Composable
internal fun SignSpecsListScreen(
	model: SignSpecsListModel,
	onBack: Callback,
	signSufficientCrypto: (seedName: String, addressKey64: String) -> Unit,
	modifier: Modifier = Modifier,
) {
	val keys = model.keys
	Column(modifier = modifier) {
		ScreenHeaderClose(
			title = stringResource(R.string.sign_specs_keys_list_title),
			onClose = onBack
		)
		LazyColumn {
			items(keys.size) { index ->
				val identity = keys[index]
				Box(Modifier.clickable {
					signSufficientCrypto(identity.seedName, identity.base58)
				}) {
					KeyCardSignature(
						model = identity,
						modifier = Modifier.padding(vertical = 8.dp, horizontal = 24.dp),
					)
				}
			}
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
private fun PreviewSignSpecsListScreen() {
	SignerNewTheme {
		SignSpecsListScreen(
			model = SignSpecsListModel.createStub(),
			onBack = {},
			signSufficientCrypto = { _, _ -> },
		)
	}
}
