package io.parity.signer.screens.error

import android.content.res.Configuration
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
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
import io.parity.signer.components.base.ScreenHeaderClose
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.textSecondary


@Composable
internal fun ErrorStateScreen(
	header: String,
	description: String,
	verbose: String,
	onBack: Callback,
	modifier: Modifier = Modifier,
) {
	Column(modifier = modifier) {
		ScreenHeaderClose(
			title = stringResource(R.string.error_state_screen_title),
			onClose = onBack
		)
		Column(
			modifier = Modifier
				.verticalScroll(rememberScrollState())
				.weight(1f, true)
		) {
			Text(
				modifier = Modifier
					.padding(horizontal = 24.dp)
					.padding(top = 16.dp, bottom = 12.dp),
				text = header,
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.TitleL,
			)
			Text(
				modifier = Modifier
					.padding(horizontal = 24.dp)
					.padding(bottom = 16.dp),
				text = description,
				color = MaterialTheme.colors.textSecondary,
				style = SignerTypeface.BodyL,
			)
			if (verbose.isNotEmpty()) {
				Text(
					modifier = Modifier
						.padding(horizontal = 24.dp),
					text = stringResource(R.string.error_state_screen_verbose_subtitle),
					color = MaterialTheme.colors.textSecondary,
					style = SignerTypeface.BodyM,
				)
				Text(
					modifier = Modifier
						.padding(horizontal = 24.dp)
						.padding(bottom = 16.dp),
					text = verbose,
					color = MaterialTheme.colors.textSecondary,
					style = SignerTypeface.BodyM,
				)
			}
		}
//		PrimaryButtonWide(
//			label = stringResource(R.string.button_next),
//			modifier = Modifier.padding(24.dp),
//			onClicked = {},
//		)
	}
}


@Preview(
	name = "light", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewErrorStateScreen() {
	Box(modifier = Modifier.size(width = 250.dp, height = 450.dp)) {
		SignerNewTheme() {
			ErrorStateScreen(
				header = "navigation failed to open signer",
				description = "cound't sign specs 0 unknown state recieved",
				verbose = "alertData=ErrorData(f=Could not decode AddressKey::multisigner:\n" +
					"Could not decode MultiSigner, variant doesn't exist\n" +
					")))",
				onBack = {},
			)
		}
	}
}

