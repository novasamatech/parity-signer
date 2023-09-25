package io.parity.signer.screens.keydetails.exportprivatekey

import android.content.res.Configuration
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.PrimaryButtonWide
import io.parity.signer.components.base.SecondaryButtonWide
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.textSecondary

@Composable
fun ConfirmExportPrivateKeyMenu(
	onClose: Callback,
	onExportPrivate: Callback,
) {
	val sidePadding = 24.dp
	Column(
		modifier = Modifier
			.fillMaxWidth()
			.padding(start = sidePadding, end = sidePadding),
		horizontalAlignment = Alignment.CenterHorizontally,
	) {
		Icon(
			painterResource(R.drawable.private_key_64),
			null,
			Modifier.padding(vertical = 32.dp),
			tint = MaterialTheme.colors.primary,
		)
		Text(
			text = stringResource(R.string.export_private_key_confirm_title),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleL,
		)
		Text(
			modifier = Modifier.padding(top = 16.dp, bottom = 24.dp),
			text = stringResource(R.string.export_private_key_confirm_text),
			color = MaterialTheme.colors.textSecondary,
			style = SignerTypeface.BodyL,
			textAlign = TextAlign.Center,
		)

		PrimaryButtonWide(
			label = stringResource(R.string.export_private_key_confirm_title),
			onClicked = onExportPrivate,
		)

		Spacer(modifier = Modifier.padding(bottom = 8.dp))

		SecondaryButtonWide(
			label = stringResource(R.string.generic_cancel),
			onClicked = onClose,
		)
		Spacer(modifier = Modifier.padding(bottom = 24.dp))
	}
}


@Preview(
	name = "light", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "themes",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewConfirmExportPrivateKeyMenu() {
	SignerNewTheme {
		ConfirmExportPrivateKeyMenu(
			{}, {}
		)
	}
}
