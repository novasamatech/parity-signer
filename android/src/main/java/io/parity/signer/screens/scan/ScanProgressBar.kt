package io.parity.signer.screens.scan

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.backgroundTertiary
import io.parity.signer.ui.theme.fill12


//todo dmitry new progress bar
@Composable
fun ScanProgressBar(captured: State<Int?>,
										total: State<Int?>,
										modifier: Modifier = Modifier) {
	val progress = (captured.value ?: 0).toFloat() / ((total.value ?: 1).toFloat())

	val innerRound = dimensionResource(id = R.dimen.qrShapeCornerRadius)
	val innerShape =
		RoundedCornerShape(innerRound, innerRound, innerRound, innerRound)
	Box(modifier = Modifier
		.fillMaxWidth(1f)
		.padding(start = 8.dp, end = 8.dp, bottom = 16.dp, top = 8.dp,)
		.background(MaterialTheme.colors.fill12, innerShape)
		.padding(start = 16.dp, end = 16.dp, bottom = 16.dp, top = 20.dp,),
	) {
			Text(text = "title")
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
	val captured = remember { mutableStateOf(20) }
	val total = remember { mutableStateOf(50) }

	SignerNewTheme {
		ScanProgressBar(captured, total)
	}
}
