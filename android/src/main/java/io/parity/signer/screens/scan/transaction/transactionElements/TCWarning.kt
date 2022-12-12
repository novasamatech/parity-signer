package io.parity.signer.screens.scan.transaction.transactionElements

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.red400
import io.parity.signer.ui.theme.textSecondary

@Composable
fun TCWarning(warning: String) {
	Row(
		modifier = Modifier.background(MaterialTheme.colors.error)
	) {
		Text(
			text = stringResource(R.string.warning_exl),
			style = SignerTypeface.BodyM,
			color = MaterialTheme.colors.red400,
		)
		Spacer(modifier = Modifier.padding(end = 8.dp))
		Text(
			text = warning,
			style = SignerTypeface.BodyM,
			color = MaterialTheme.colors.red400,
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
private fun PreviewTCWarning() {
	SignerNewTheme {
		TCWarning("Content of warning!")
	}
}
