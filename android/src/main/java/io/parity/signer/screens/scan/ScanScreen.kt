package io.parity.signer.screens.scan

import android.content.res.Configuration
import androidx.camera.lifecycle.ProcessCameraProvider
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.material.AlertDialog
import androidx.compose.material.Button
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalInspectionMode
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.lifecycle.viewmodel.compose.viewModel
import com.google.accompanist.permissions.ExperimentalPermissionsApi
import com.google.accompanist.permissions.isGranted
import com.google.accompanist.permissions.rememberPermissionState
import com.google.accompanist.permissions.shouldShowRationale
import io.parity.signer.components.KeepScreenOn
import io.parity.signer.components.base.CloseIcon
import io.parity.signer.models.Callback
import io.parity.signer.models.KeySetDetailsModel
import io.parity.signer.screens.scan.items.CameraMultiSignIcon
import io.parity.signer.ui.theme.SignerNewTheme
import kotlinx.coroutines.launch

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


	Box(
		Modifier
			.fillMaxSize(1f)) {
		ScanHeader(onClose)
		CameraView()
	}
	KeepScreenOn()
}

@OptIn(ExperimentalPermissionsApi::class)
@Composable
private fun CameraView() {
	if (LocalInspectionMode.current) return

	val context = LocalContext.current
	val cameraProviderFuture = remember { ProcessCameraProvider.getInstance(context) }

	val cameraPermissionState = rememberPermissionState(android.Manifest.permission.CAMERA)
	if (cameraPermissionState.status.isGranted) {
		Text("Camera permission Granted")
	} else {
		val scope = rememberCoroutineScope()
		Column {
			if (cameraPermissionState.status.shouldShowRationale) {
				AlertDialog(
					onDismissRequest = { },
					confirmButton = {
						Button(onClick = { scope.launch { cameraPermissionState.launchPermissionRequest() } }) {
							Text(text = "OK")
						}
					},
					title = { Text(text = "Camera required!")},
					text = { Text(text = "To work with QR code we need camera permission, this is main functionality of this app!")},
				)
			} else {
				Text("Camera not available")
				scope.launch { cameraPermissionState.launchPermissionRequest() }
			}
		}
	}
}

@Composable
fun ScanHeader(onClose: Callback) {
	Row(Modifier.fillMaxWidth(1f)) {
		CloseIcon(modifier = Modifier.padding(vertical = 20.dp),
			onCloseClicked = onClose)
		Spacer(modifier = Modifier.weight(1f))
		CameraMultiSignIcon(isEnabled = false,
		onClick = {}) //todo Dmitry
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
