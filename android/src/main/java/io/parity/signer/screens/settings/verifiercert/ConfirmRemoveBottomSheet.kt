package io.parity.signer.screens.settings.verifiercert

import android.content.res.Configuration
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.AnnotatedString
import androidx.compose.ui.text.SpanStyle
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.tooling.preview.Preview
import io.parity.signer.R
import io.parity.signer.components.base.BottomSheetConfirmDialog
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.pink300


@Composable
internal fun ConfirmRemoveCertificateBottomSheet(
	onCancel: Callback,
	onRemoveCertificate: Callback,
) {
	//todo dmitry finish new text
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

	BottomSheetConfirmDialog(
		title = stringResource(R.string.network_details_remove_confirm_title),
		message = stringResource(R.string.network_details_remove_confirm_description),
		ctaLabel = stringResource(R.string.remove_key_set_confirm_cta),
		onCancel = onCancel,
		onCta = onRemoveCertificate,
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
private fun PreviewConfirmRemoveNetworkBottomSheet() {
	SignerNewTheme {
		ConfirmRemoveCertificateBottomSheet(
			{}, {},
		)
	}
}


