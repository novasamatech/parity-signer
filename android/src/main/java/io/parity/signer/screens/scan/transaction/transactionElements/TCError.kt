package io.parity.signer.screens.scan.transaction.transactionElements

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.backgroundDanger
import io.parity.signer.ui.theme.pink300

@Composable
fun TCError(error: String) {
	Row(
		modifier = Modifier.background(
			color = MaterialTheme.colors.backgroundDanger,
			shape = RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius))
		),
		horizontalArrangement = Arrangement.spacedBy(8.dp),
	) {
		Text(
			text = stringResource(R.string.transaction_field_error),
			style = SignerTypeface.BodyM,
			color = MaterialTheme.colors.pink300
		)
		Text(
			text = error,
			style = SignerTypeface.BodyM,
			color = MaterialTheme.colors.pink300
		)
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
private fun PreviewTCError() {
	SignerNewTheme {
		Column {
			TCError("Something went wrong!")
//			SignerDivider()
		}
	}
}
