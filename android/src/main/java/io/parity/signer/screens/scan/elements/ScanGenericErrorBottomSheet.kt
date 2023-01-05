package io.parity.signer.screens.scan.elements

import android.content.res.Configuration
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.SecondaryButtonWide
import io.parity.signer.models.Callback
import io.parity.signer.ui.theme.*

@Composable
fun ScanErrorBottomSheet(
	errorMessage: String,
	onOK: Callback,
) {
	Column(
		Modifier
			.fillMaxWidth(1f)
			.verticalScroll(rememberScrollState())
	) {
		Text(
			text = stringResource(R.string.transaction_generic_error_title),
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
			text = stringResource(R.string.transaction_generic_error_description),
			color = MaterialTheme.colors.textSecondary,
			style = SignerTypeface.BodyM,
			modifier = Modifier.padding(horizontal = 32.dp),
		)

		Surface(
			shape = RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius)),
			color = MaterialTheme.colors.fill6,
			border = BorderStroke(1.dp, color = MaterialTheme.colors.fill12),
			modifier = Modifier
				.fillMaxWidth(1f)
				.padding(horizontal = 24.dp)
				.padding(top = 24.dp)
		) {
			Text(
				text = errorMessage,
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.BodyL,
				modifier = Modifier.padding(vertical = 12.dp, horizontal = 16.dp)
			)
		}
		SecondaryButtonWide(
			label = stringResource(id = R.string.generic_ok),
			modifier = Modifier.padding(24.dp),
			withBackground = true,
			onClicked = onOK,
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
private fun PreviewScanErrorBottomSheet() {
	SignerNewTheme {
		ScanErrorBottomSheet(
			"Bad input data. Metadata for westend9330 is already in the database.",
			{})
	}
}
