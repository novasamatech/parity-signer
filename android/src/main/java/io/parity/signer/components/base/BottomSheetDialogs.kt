package io.parity.signer.components.base

import android.content.res.Configuration
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.textSecondary


@Composable
fun BottomSheetConfirmDialog(
	title: String,
	message: String,
	ctaLabel: String,
	onCancel: Callback,
	onCta: Callback,
) {
	val sidePadding = 24.dp
	Column(
		modifier = Modifier
			.fillMaxWidth()
			.verticalScroll(rememberScrollState())
			.padding(start = sidePadding, end = sidePadding, top = 32.dp),
	) {

		Text(
			modifier = Modifier.fillMaxWidth(1f),
			text = title,
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleL,
			textAlign = TextAlign.Center,
		)
		Text(
			modifier = Modifier
				.fillMaxWidth(1f)
				.padding(
					top = 16.dp, bottom = 24.dp,
					start = 8.dp, end = 8.dp
				),
			text = message,
			color = MaterialTheme.colors.textSecondary,
			style = SignerTypeface.BodyL,
			textAlign = TextAlign.Center,
		)
		RowButtonsBottomSheet(
			labelCancel = stringResource(R.string.generic_cancel),
			labelCta = ctaLabel,
			onClickedCancel = onCancel,
			onClickedCta = onCta,
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
private fun PreviewBottomSheetConfirmDialog() {
	SignerNewTheme {
		BottomSheetConfirmDialog(
			"Title", "Message long description message",
			"Cta lable",
			{}, {},
		)
	}
}
