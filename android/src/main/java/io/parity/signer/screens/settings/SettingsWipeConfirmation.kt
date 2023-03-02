package io.parity.signer.screens.settings

import android.content.res.Configuration
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.AnnotatedString
import androidx.compose.ui.text.SpanStyle
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.PrimaryButtonWide
import io.parity.signer.components.base.SecondaryButtonWide
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.pink300
import io.parity.signer.ui.theme.textSecondary


@Composable
fun SettingsWipeAllConfirmation(
	onWipe: Callback,
	onCancel: Callback,
) {
	val sidePadding = 24.dp
	Column(
		modifier = Modifier
			.fillMaxWidth()
			.padding(start = sidePadding, end = sidePadding, top = 32.dp),
	) {

		Text(
			modifier = Modifier.fillMaxWidth(1f),
			text = stringResource(R.string.settings_confirmation_wipe_all_title),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleL,
		)

		val warning =
			stringResource(R.string.settings_confirmation_wipe_all_message_highlited_warning).uppercase()
		val message =
			stringResource(R.string.settings_confirmation_wipe_all_message, warning)
		val startPosition = message.indexOf(warning)
		val spanStyles = listOf(
			AnnotatedString.Range(
				SpanStyle(
					fontWeight = FontWeight.Bold,
					color = MaterialTheme.colors.pink300
				),
				start = startPosition,
				end = startPosition + warning.length
			)
		)
		Text(
			modifier = Modifier
				.fillMaxWidth(1f)
				.padding(
					top = 16.dp, bottom = 24.dp,
				),
			text = AnnotatedString(message, spanStyles),
			color = MaterialTheme.colors.textSecondary,
			style = SignerTypeface.BodyL,
		)
		PrimaryButtonWide(
			label = stringResource(R.string.settings_confirmation_wipe_all_message_yes_label),
			activeBackground = Color.Red,
			onClicked = onWipe
		)
		SecondaryButtonWide(
			label = stringResource(R.string.generic_cancel),
			textColor = MaterialTheme.colors.primary,
			onClicked = onCancel,
		)
		Spacer(modifier = Modifier.padding(bottom = 24.dp))
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
private fun PreviewSettingsWipeAllConfirmation() {
	SignerNewTheme {
		SettingsWipeAllConfirmation({}, {})
	}
}
