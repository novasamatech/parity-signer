package io.parity.signer.screens.scan

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.LinearProgressIndicator
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.models.Callback
import io.parity.signer.ui.theme.*


@Composable
fun ScanProgressBar(
	captured: Int,
	total: Int?,
	onCancel: Callback,
) {
	val progress =
		captured.toFloat() / (total ?: 1).toFloat()

	val innerRound = dimensionResource(id = R.dimen.qrShapeCornerRadius)
	val innerShape =
		RoundedCornerShape(innerRound, innerRound, innerRound, innerRound)
	Column(
		modifier = Modifier
			.fillMaxWidth(1f)
			.padding(start = 8.dp, end = 8.dp, bottom = 16.dp, top = 8.dp)
			.background(MaterialTheme.colors.fill12, innerShape)
			.background(MaterialTheme.colors.backgroundTertiary, innerShape)
			.padding(start = 16.dp, end = 16.dp, bottom = 16.dp, top = 20.dp),
	) {
		Row(verticalAlignment = Alignment.CenterVertically) {
			Column(Modifier.weight(1f)) {
				Text(
					text = stringResource(R.string.scan_progress_bar_title),
					color = MaterialTheme.colors.primary,
					style = SignerTypeface.BodyL,
				)
				Text(
					text = stringResource(R.string.scan_progress_bar_progress, captured, total ?: -1),
					color = MaterialTheme.colors.textTertiary,
					style = SignerTypeface.CaptionM,
				)
			}
			Text(
				text = stringResource(id = R.string.generic_cancel),
				color = MaterialTheme.colors.pink500,
				style = SignerTypeface.LabelM,
				modifier = Modifier
					.padding(8.dp)
					.clickable(onClick = onCancel),
			)
		}
		Spacer(modifier = Modifier.padding(top = 12.dp))
		LinearProgressIndicator(
			progress = progress,
			modifier = Modifier.fillMaxWidth(1f),
			color = MaterialTheme.colors.pink300,
			backgroundColor = MaterialTheme.colors.fill18,
		)
	}
}


@Preview(
	name = "light", group = "general", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xB0FFFFFF,
)
@Preview(
	name = "dark", group = "general",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewScanProgressBar() {
	SignerNewTheme {
		ScanProgressBar(20, 50, {})
	}
}
