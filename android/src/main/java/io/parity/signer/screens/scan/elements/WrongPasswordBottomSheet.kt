package io.parity.signer.screens.scan.elements

import android.content.res.Configuration
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.SecondaryButtonBottomSheet
import io.parity.signer.models.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.textSecondary


@Composable
internal fun WrongPasswordBottomSheet(onOk: Callback) {
	Column(Modifier.fillMaxWidth(1f)) {
		Text(
			text = stringResource(R.string.wrong_password_title),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleM,
			modifier = Modifier.padding(
				top = 32.dp,
				bottom = 8.dp,
				start = 32.dp,
				end = 32.dp
			),
		)
		Text(
			text = stringResource(R.string.wrong_password_message),
			color = MaterialTheme.colors.textSecondary,
			style = SignerTypeface.BodyM,
			modifier = Modifier.padding(horizontal = 32.dp),
		)
		SecondaryButtonBottomSheet(
			label = stringResource(id = R.string.generic_ok),
			modifier = Modifier.padding(24.dp),
			withBackground = true,
			onClicked = onOk,
		)
	}
}

@Preview(
	name = "light theme",
	uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true,
)
@Preview(
	name = "dark theme",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	backgroundColor = 0xFFFFFFFF
)
@Composable
private fun PrevieWrongPasswordBottomSheet() {
	SignerNewTheme {
		WrongPasswordBottomSheet({})
	}
}
