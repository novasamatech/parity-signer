package io.parity.signer.bottomsheets

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.RowButtonsBottomSheet
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.backgroundTertiary

@Composable
internal fun ProceedEmptyKeysetConfirmation(
	onCancel: Callback,
	onProceed: Callback,
) {

	Column(Modifier.background(MaterialTheme.colors.backgroundTertiary))
	{
		Text(
			text = stringResource(R.string.key_set_no_networks_title),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleL,
			modifier = Modifier.padding(horizontal = 24.dp)
				.padding(top = 24.dp),
		)
		Text(
			text = stringResource(R.string.key_set_no_networks_subtitle),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.BodyL,
			modifier = Modifier
				.padding(horizontal = 24.dp)
				.padding(top = 8.dp)
		)
		RowButtonsBottomSheet(
			modifier = Modifier.padding(24.dp),
			labelCancel = stringResource(id = R.string.generic_cancel),
			labelCta = stringResource(id = R.string.button_next),
			onClickedCancel = onCancel,
			onClickedCta = onProceed,
			isCtaEnabled = true,
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
private fun PreviewProceedEmptyKeysetConfirmation() {
	SignerNewTheme {
		ProceedEmptyKeysetConfirmation({}, {})
	}
}
