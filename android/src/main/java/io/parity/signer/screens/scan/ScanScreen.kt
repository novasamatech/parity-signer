package io.parity.signer.screens.scan

import android.content.res.Configuration
import android.view.ViewGroup
import androidx.camera.core.CameraSelector
import androidx.camera.core.ImageAnalysis
import androidx.camera.lifecycle.ProcessCameraProvider
import androidx.camera.view.PreviewView
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.material.AlertDialog
import androidx.compose.material.Button
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalInspectionMode
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.compose.ui.viewinterop.AndroidView
import androidx.core.content.ContextCompat
import androidx.lifecycle.viewmodel.compose.viewModel
import com.google.accompanist.permissions.ExperimentalPermissionsApi
import com.google.accompanist.permissions.isGranted
import com.google.accompanist.permissions.rememberPermissionState
import com.google.accompanist.permissions.shouldShowRationale
import com.google.mlkit.vision.barcode.BarcodeScannerOptions
import com.google.mlkit.vision.barcode.BarcodeScanning
import com.google.mlkit.vision.barcode.common.Barcode
import io.parity.signer.R
import io.parity.signer.components.KeepScreenOn
import io.parity.signer.components.base.CloseIcon
import io.parity.signer.models.Callback
import io.parity.signer.models.FeatureOption
import io.parity.signer.models.KeySetDetailsModel
import io.parity.signer.screens.scan.items.CameraLightIcon
import io.parity.signer.screens.scan.items.CameraMultiSignIcon
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.TypefaceNew
import io.parity.signer.uniffi.MTransaction
import kotlinx.coroutines.launch

@Composable
fun ScanScreen(
	onClose: Callback,
	onNavigateToTransaction: (MTransaction) -> Unit,
) {
	val viewModel: CameraViewModel = viewModel()

	val captured = viewModel.captured.collectAsState()
	val total = viewModel.total.collectAsState()
	val isMultimode = viewModel.isMultiscanMode.collectAsState()

	Box(
		Modifier
			.fillMaxSize(1f)
			.background(MaterialTheme.colors.background)
	) {
		CameraViewPermission(viewModel)
		ScanHeader(Modifier.statusBarsPadding(), onClose)
		if (captured.value != null) {
			ScanProgressBar(captured, total) { viewModel.resetScanValues() }
		}
		Column(
			Modifier
				.fillMaxSize(1f)
				.padding(horizontal = 48.dp),
		) {
			Spacer(modifier = Modifier.weight(1f))
			Text(
				text = stringResource(
					if (isMultimode.value) {
						R.string.camera_screen_header_multimode
					} else {
						R.string.camera_screen_header_single
					}
				),
				color = Color.White,
				style = TypefaceNew.TitleL,
				textAlign = TextAlign.Center,
				modifier = Modifier.fillMaxWidth(1f),
			)
			Spacer(modifier = Modifier.padding(bottom = 12.dp))
			Text(
				text = stringResource(
					if (isMultimode.value) {
						R.string.camera_screen_description_multimode
					} else {
						R.string.camera_screen_description_single
					}
				),
				color = Color.White,
				style = TypefaceNew.TitleS,
				textAlign = TextAlign.Center,
				modifier = Modifier.fillMaxWidth(1f),
			)
			Spacer(modifier = Modifier.padding(bottom = 76.dp))
		}
	}
	KeepScreenOn()
}


@OptIn(ExperimentalPermissionsApi::class)
@Composable
private fun CameraViewPermission(viewModel: CameraViewModel) {
	if (LocalInspectionMode.current) return

	val cameraPermissionState =
		rememberPermissionState(android.Manifest.permission.CAMERA)
	if (cameraPermissionState.status.isGranted) {
		CameraViewInternal(viewModel)
		TransparentClipLayout()
	} else {
		Column {
			if (cameraPermissionState.status.shouldShowRationale) {
				AlertDialog(
					onDismissRequest = { },
					confirmButton = {
						val scope = rememberCoroutineScope()
						Button(onClick = { scope.launch { cameraPermissionState.launchPermissionRequest() } }) {
							Text(text = "OK")
						}
					},
					title = { Text(text = "Camera required!") },
					text = { Text(text = "To work with QR code we need camera permission, this is main functionality of this app!") },
				)
			} else {
				Text("Camera permission not granted")
				LaunchedEffect(key1 = Unit) {
					launch { cameraPermissionState.launchPermissionRequest() }
				}
			}
		}
	}
}

@Composable
private fun CameraViewInternal(viewModel: CameraViewModel) {
	val lifecycleOwner = LocalLifecycleOwner.current
	val context = LocalContext.current
	val cameraProviderFuture =
		remember { ProcessCameraProvider.getInstance(context) }

	AndroidView(
		factory = { context ->
			val executor = ContextCompat.getMainExecutor(context)
			val previewView = PreviewView(context).apply {
				this.scaleType = PreviewView.ScaleType.FILL_CENTER
				layoutParams = ViewGroup.LayoutParams(
					ViewGroup.LayoutParams.MATCH_PARENT,
					ViewGroup.LayoutParams.MATCH_PARENT
				)
			}
			// mlkit docs: The default option is not recommended because it tries
			// to scan all barcode formats, which is slow.
			val options = BarcodeScannerOptions.Builder()
				.setBarcodeFormats(Barcode.FORMAT_QR_CODE).build()

			val barcodeScanner = BarcodeScanning.getClient(options)

			cameraProviderFuture.addListener({
				val cameraProvider = cameraProviderFuture.get()

				val preview = androidx.camera.core.Preview.Builder().build().also {
					it.setSurfaceProvider(previewView.surfaceProvider)
				}

				val cameraSelector = CameraSelector.Builder()
					.requireLensFacing(CameraSelector.LENS_FACING_BACK)
					.build()

				val imageAnalysis = ImageAnalysis.Builder()
					.setBackpressureStrategy(ImageAnalysis.STRATEGY_KEEP_ONLY_LATEST)
					.build()
					.apply {
						setAnalyzer(executor) { imageProxy ->
							viewModel.processFrame(barcodeScanner, imageProxy)
						}
					}

				cameraProvider.unbindAll()
				cameraProvider.bindToLifecycle(
					lifecycleOwner,
					cameraSelector,
					imageAnalysis,
					preview
				)
			}, executor)
			previewView
		},
	)
}


@Composable
private fun ScanHeader(
	modifier: Modifier = Modifier,
	onClose: Callback,
) {
	val viewModel: CameraViewModel = viewModel()
	val tourchEnabled = viewModel.isTourchEnabled.collectAsState()
	Row(
		modifier
			.fillMaxWidth(1f)
			.padding(horizontal = 16.dp)
	) {
		CloseIcon(
			onCloseClicked = onClose
		)
		Spacer(modifier = Modifier.weight(1f))
		if (FeatureOption.MULTI_TRANSACTION_CAMERA.isEnabled()) {
			CameraMultiSignIcon(isEnabled = false,
				onClick = {}) //todo Dmitry
		}
		Spacer(modifier = Modifier.padding(end = 8.dp))
		CameraLightIcon(isEnabled = tourchEnabled.value == true,
			onClick = {viewModel.isTourchEnabled.value = !tourchEnabled.value}) //todo Dmitry

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
			ScanScreen({}, {_->})
		}
	}
}
