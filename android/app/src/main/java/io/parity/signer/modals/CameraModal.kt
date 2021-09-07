package io.parity.signer.modals

import androidx.camera.core.CameraSelector
import androidx.camera.core.ImageAnalysis
import androidx.camera.core.Preview
import androidx.camera.lifecycle.ProcessCameraProvider
import androidx.camera.view.PreviewView
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.compose.ui.viewinterop.AndroidView
import androidx.core.content.ContextCompat
import com.google.mlkit.vision.barcode.BarcodeScanning
import io.parity.signer.models.SignerDataModel

@Composable
fun CameraModal(signerDataModel: SignerDataModel) {
	val lifecycleOwner = LocalLifecycleOwner.current
	val context = LocalContext.current
	val cameraProviderFuture =
		remember { ProcessCameraProvider.getInstance(context) }

	Box(
		modifier = Modifier
			.fillMaxSize()
	) {
		//TODO: use all the cores needed to make this smooth
		AndroidView(
			factory = { context ->
				val executor = ContextCompat.getMainExecutor(context)
				val previewView = PreviewView(context)
				val barcodeScanner = BarcodeScanning.getClient()
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
							setAnalyzer(executor, ImageAnalysis.Analyzer { imageProxy ->
								signerDataModel.processFrame(barcodeScanner, imageProxy)
							})
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
			modifier = Modifier.fillMaxSize()
		)

	}
}
