package io.parity.signer.screens.scan.errors

import android.content.res.Configuration
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.ui.theme.*


@Composable
fun TransactionErrorEmbedded(errors: String, modifier: Modifier = Modifier) {
	Surface(
		modifier = modifier.fillMaxWidth(1f),
		border = BorderStroke(1.dp, MaterialTheme.colors.fill12),
		shape = RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius)),
		color = MaterialTheme.colors.red500fill12,
	) {
		Text(
			text = errors,
			color = MaterialTheme.colors.red500,
			style = SignerTypeface.BodyM,
			modifier = Modifier.padding(16.dp),
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
private fun PreviewTransactionErrors() {
	SignerNewTheme {
		TransactionErrorEmbedded(
			"Bad input data. Metadata for westend9330 is already in the database.",
		)
	}
}
