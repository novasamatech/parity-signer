package io.parity.signer.screens.scan

import android.content.res.Configuration
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.size
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.models.Callback
import io.parity.signer.models.EmptyNavigator
import io.parity.signer.models.KeySetDetailsModel
import io.parity.signer.ui.theme.SignerNewTheme

//todo dmitry add kep scree on from phrase box like in old screen
@Composable
fun ScanScreen(
	onClose: Callback
) {
	Box() {
		ScanHeader(onClose)
	}
}


@Composable
fun ScanHeader(onClose: Callback) {
	Row() {

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
private fun PreviewScanScreen() {
	val mockModel = KeySetDetailsModel.createStub()
	SignerNewTheme {
		Box(modifier = Modifier.size(350.dp, 550.dp)) {
			ScanScreen(mockModel, EmptyNavigator(), {})
		}
	}
}
