package io.parity.signer.screens.scan

import android.content.res.Configuration
import androidx.camera.lifecycle.ProcessCameraProvider
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalInspectionMode
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.lifecycle.viewmodel.compose.viewModel
import io.parity.signer.components.base.CloseIcon
import io.parity.signer.models.Callback
import io.parity.signer.models.KeySetDetailsModel
import io.parity.signer.ui.theme.SignerNewTheme

//todo dmitry add kep scree on from phrase box like in old screen
@Composable
fun ScanScreen(
	onClose: Callback
) {
	val interactor: CameraViewModel = viewModel()

	val progress = interactor.progress.observeAsState()
	val captured = interactor.captured.observeAsState()
	val total = interactor.total.observeAsState()

	val lifecycleOwner = LocalLifecycleOwner.current
	val context = LocalContext.current
	val cameraProviderFuture =
		if (!LocalInspectionMode.current) {
			remember { ProcessCameraProvider.getInstance(context) }
		} else null

	Box() {
		ScanHeader(onClose)
	}
}


@Composable
fun ScanHeader(onClose: Callback) {
	Row() {
		CloseIcon(modifier = Modifier.padding(vertical = 20.dp),
			onCloseClicked = onClose)
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
			ScanScreen({})
		}
	}
}
