package io.parity.signer.modals

import android.widget.ProgressBar
import androidx.camera.core.CameraSelector
import androidx.camera.core.ImageAnalysis
import androidx.camera.core.Preview
import androidx.camera.lifecycle.ProcessCameraProvider
import androidx.camera.view.PreviewView
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.LinearProgressIndicator
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.compose.ui.unit.dp
import androidx.compose.ui.viewinterop.AndroidView
import androidx.core.content.ContextCompat
import com.google.mlkit.vision.barcode.BarcodeScanning
import io.parity.signer.components.ScanProgressBar
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.processFrame
import io.parity.signer.ui.theme.Crypto400
import io.parity.signer.ui.theme.ParitySignerTheme
import kotlin.contracts.Effect

@Composable
fun CameraModal(signerDataModel: SignerDataModel) {
	val lifecycleOwner = LocalLifecycleOwner.current
	val context = LocalContext.current
	val cameraProviderFuture =
		remember { ProcessCameraProvider.getInstance(context) }

	Column (Modifier.fillMaxSize()) {
		Box(
			Modifier.padding(8.dp)
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
								setAnalyzer(executor, { imageProxy ->
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
				Modifier
					.padding(bottom = 24.dp)
					.border(
						BorderStroke(1.dp, MaterialTheme.colors.Crypto400),
						RoundedCornerShape(8.dp)
					)
					.clip(RoundedCornerShape(8.dp))
			)
			Column(
				verticalArrangement = Arrangement.Bottom,
				modifier = Modifier.fillMaxSize()
			) {
				ScanProgressBar(signerDataModel = signerDataModel)
			}
		}
	}
}
