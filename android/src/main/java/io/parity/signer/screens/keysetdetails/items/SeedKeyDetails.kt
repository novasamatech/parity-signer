package io.parity.signer.screens.keysetdetails.items

import android.content.res.Configuration
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.components.networkicon.IdentIconImage
import io.parity.signer.components.sharedcomponents.ShowBase58Collapsed
import io.parity.signer.domain.Callback
import io.parity.signer.domain.KeyModel
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface


@Composable
fun SeedKeyDetails(
	model: KeyModel,
	onShowRoot: Callback,
	modifier: Modifier = Modifier,
) {
	Column(
		modifier = modifier
			.fillMaxWidth()
			.clickable(onClick = onShowRoot),
		horizontalAlignment = Alignment.CenterHorizontally,
	) {
		IdentIconImage(
			identIcon = model.identicon,
			modifier = Modifier.clickable(onClick = onShowRoot),
			size = 56.dp
		)
		Text(
			text = model.seedName,
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleXl,
			textAlign = TextAlign.Center
		)
		ShowBase58Collapsed(model.base58, Modifier.padding(top = 8.dp))
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
		SeedKeyDetails(KeyModel.createStub()
			.copy(identicon = PreviewData.Identicon.jdenticonIcon), {})
	}
}
