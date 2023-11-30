package io.parity.signer.screens.scan.transaction.dynamicderivations

import android.content.res.Configuration
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.RowButtonsBottomSheet
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.textSecondary

@Composable
internal fun AddDDConfirmCloseBottomSheet(
	onConfirm: Callback,
	onCancel: Callback,
) {
	val sidePadding = 24.dp
	Column(
		modifier = Modifier
			.fillMaxWidth()
			.padding(start = sidePadding, end = sidePadding),
		horizontalAlignment = Alignment.CenterHorizontally,
	) {
		Text(
			text = stringResource(R.string.confirm_dynamic_derivation_header),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleL,
			textAlign = TextAlign.Center,
			modifier = Modifier.padding(top = 24.dp)
		)
		Text(
			modifier = Modifier.padding(top = 12.dp, bottom = 8.dp),
			text = stringResource(R.string.confirm_dynamic_derivation_message),
			color = MaterialTheme.colors.textSecondary,
			style = SignerTypeface.BodyL,
			textAlign = TextAlign.Center,
		)
		RowButtonsBottomSheet(
			labelCancel = stringResource(R.string.generic_cancel),
			labelCta = stringResource(R.string.confirm_dynamic_derivation_cta),
			onClickedCancel = onCancel,
			onClickedCta = onConfirm,
			modifier = Modifier.padding(vertical = 16.dp),
		)
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
		AddDDConfirmCloseBottomSheet({}, {})
	}
}
