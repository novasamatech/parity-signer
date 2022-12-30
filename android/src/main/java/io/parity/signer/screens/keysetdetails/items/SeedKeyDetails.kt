package io.parity.signer.screens.keysetdetails.items

import android.content.res.Configuration
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import io.parity.signer.components.sharedcomponents.ShowBase58Collapsible
import io.parity.signer.models.KeyModel
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface


@Composable
fun SeedKeyDetails(
	model: KeyModel,
	modifier: Modifier = Modifier
) {
	Column(modifier = modifier.fillMaxWidth(),
		horizontalAlignment = Alignment.CenterHorizontally) {
		Text(
			text = model.seedName,
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleXl
		)
		ShowBase58Collapsible(model.base58)
	}
}


@Preview(
	name = "light",
	uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true,
)
@Preview(
	name = "dark",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	backgroundColor = 0xFFFFFFFF
)
@Composable
private fun PreviewKeySeedCard() {
	SignerNewTheme {
		SeedKeyDetails(KeyModel.createStub())
	}
}
