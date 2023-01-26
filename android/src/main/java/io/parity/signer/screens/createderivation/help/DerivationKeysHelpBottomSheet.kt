package io.parity.signer.screens.createderivation.help

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
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.SecondaryButtonWide
import io.parity.signer.models.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.backgroundTertiary
import io.parity.signer.ui.theme.textSecondary

@Composable
fun DerivationKeysHelpBottomSheet(
	onClose: Callback,
) {
	Column(
		Modifier
			.background(MaterialTheme.colors.backgroundTertiary)
			.verticalScroll(rememberScrollState())
			.padding(horizontal = 24.dp)
	) {

		Text(
			text = stringResource(R.string.derivation_help_keys_title),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleS,
			modifier = Modifier.padding(vertical = 12.dp),
		)
		Text(
			text = stringResource(R.string.derivation_help_keys_message1) +
				"\n\n" +
				stringResource(R.string.derivation_help_keys_message2) +
				"\n\n" +
				stringResource(R.string.derivation_help_keys_message3),
			color = MaterialTheme.colors.textSecondary,
			style = SignerTypeface.BodyM,
		)
		SecondaryButtonWide(
			label = stringResource(R.string.button_label_got_it),
			modifier = Modifier.padding(
				top = 24.dp,
				bottom = 32.dp,
				start = 32.dp,
				end = 32.dp
			),
			withBackground = true,
			onClicked = onClose
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
private fun PreviewDerivationKeysHelpBottomSheet() {
	SignerNewTheme {
		DerivationKeysHelpBottomSheet({})
	}
}
