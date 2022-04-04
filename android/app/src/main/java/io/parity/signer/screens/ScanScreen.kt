package io.parity.signer.screens

import androidx.camera.core.CameraSelector
import androidx.camera.core.ImageAnalysis
import androidx.camera.core.ImageProxy
import androidx.camera.core.Preview
import androidx.camera.lifecycle.ProcessCameraProvider
import androidx.camera.view.PreviewView
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.compose.ui.unit.dp
import androidx.compose.ui.viewinterop.AndroidView
import androidx.core.content.ContextCompat
import com.google.mlkit.vision.barcode.BarcodeScanner
import com.google.mlkit.vision.barcode.BarcodeScanning
import io.parity.signer.components.ScanProgressBar
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.processFrame
import io.parity.signer.ui.theme.Crypto400

/**
 * Main scanner screen. One of navigation roots.
 */
@Composable
fun ScanScreen(
	handleCameraPermissions: () -> Unit,
	processFrame: (BarcodeScanner, ImageProxy) -> Unit,
	progress: State<Float?>,
	captured: State<Int?>,
	total: State<Int?>,
	resetScan: () -> Unit
) {
	val lifecycleOwner = LocalLifecycleOwner.current
	val context = LocalContext.current
	val cameraProviderFuture =
		remember { ProcessCameraProvider.getInstance(context) }

	Column (
		Modifier
			.fillMaxSize()
			.verticalScroll(rememberScrollState())) {
		Box(
			Modifier.padding(8.dp)
		) {
			//TODO: use all the cores needed to make this smooth
			AndroidView(
				factory = { context ->
					val executor = ContextCompat.getMainExecutor(context)
					val previewView = PreviewView(context)
					val barcodeScanner = BarcodeScanning.getClient()

					//This might be done more elegantly, if needed.
					//But it's pretty obvious that the app needs camera
					//and why; also it just works so far and code is tiny.
					handleCameraPermissions()

					cameraProviderFuture.addListener({
						val cameraProvider = cameraProviderFuture.get()

						val preview = Preview.Builder().build().also {
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
									processFrame(barcodeScanner, imageProxy)
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
				Modifier
					.padding(bottom = 24.dp)
					.border(
						BorderStroke(1.dp, MaterialTheme.colors.Crypto400),
						RoundedCornerShape(8.dp)
					)
					.clip(RoundedCornerShape(8.dp))
			)

		}
		Column(
			verticalArrangement = Arrangement.Bottom,
			modifier = Modifier.fillMaxSize()
		) {
			ScanProgressBar(progress, captured, total, resetScan)
		}
	}
	DisposableEffect(Unit) {
		resetScan()
		onDispose {  }
	}
}
