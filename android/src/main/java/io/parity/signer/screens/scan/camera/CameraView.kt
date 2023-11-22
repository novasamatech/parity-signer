package io.parity.signer.screens.scan.camera

import android.util.Log
import android.util.Rational
import android.util.Size
import android.view.ViewGroup
import androidx.camera.core.*
import androidx.camera.lifecycle.ProcessCameraProvider
import androidx.camera.view.PreviewView
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.platform.LocalConfiguration
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.compose.ui.viewinterop.AndroidView
import androidx.core.content.ContextCompat
import com.google.mlkit.vision.barcode.BarcodeScannerOptions
import com.google.mlkit.vision.barcode.BarcodeScanning
import com.google.mlkit.vision.barcode.common.Barcode
import io.parity.signer.ui.helpers.afterMeasured
import kotlinx.coroutines.launch
import java.util.concurrent.Executors
import java.util.concurrent.TimeUnit


@Composable
internal fun CameraViewInternal(viewModel: CameraViewModel) {
	val lifecycleOwner = LocalLifecycleOwner.current
	val context = LocalContext.current
	val cameraProviderFuture = remember {
		ProcessCameraProvider.getInstance(context)
	}
	val coroutineScope = rememberCoroutineScope()
	val configuration = LocalConfiguration.current
	val backgroundExecutor = remember {
		Executors.newCachedThreadPool()
	}

	AndroidView(
		factory = { context ->
			val executor = ContextCompat.getMainExecutor(context)
			val previewView = PreviewView(context).apply {
				this.scaleType = PreviewView.ScaleType.FILL_CENTER
				layoutParams = ViewGroup.LayoutParams(
					ViewGroup.LayoutParams.MATCH_PARENT,
					ViewGroup.LayoutParams.MATCH_PARENT,
				)
			}
			// mlkit docs: The default option is not recommended because it tries
			// to scan all barcode formats, which is slow.
			val options = BarcodeScannerOptions.Builder()
				.setBarcodeFormats(Barcode.FORMAT_QR_CODE)
				.setExecutor(backgroundExecutor)
				.build()
			val barcodeScanner = BarcodeScanning.getClient(options)

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
					.setOutputImageFormat(ImageAnalysis.OUTPUT_IMAGE_FORMAT_YUV_420_888)
					.setTargetResolution(Size(1920, 1080))
					.build()
					.apply {
						setAnalyzer(executor) { imageProxy ->
							viewModel.processFrame(barcodeScanner, imageProxy)
						}
					}

				val viewPort =
					ViewPort.Builder(
						Rational(
							configuration.screenWidthDp,
							configuration.screenHeightDp
						), previewView.display.rotation
					)
						.build()

				val useCaseGroup = UseCaseGroup.Builder()
					.addUseCase(preview)
					.addUseCase(imageAnalysis)
					.setViewPort(viewPort)
					.build()

				cameraProvider.unbindAll()
				val camera = cameraProvider.bindToLifecycle(
					lifecycleOwner,
					cameraSelector,
					useCaseGroup
				)
				//torch control
				if (camera.cameraInfo.hasFlashUnit()) {
					coroutineScope.launch {
						viewModel.isTorchEnabled.collect {
							camera.cameraControl.enableTorch(it)
						}
					}
				}
				//autofocus
				previewView.afterMeasured {
					val autoFocusPoint = SurfaceOrientedMeteringPointFactory(1f, 1f)
						.createPoint(.5f, .5f)
					try {
						val autoFocusAction = FocusMeteringAction.Builder(
							autoFocusPoint,
							FocusMeteringAction.FLAG_AF
						).apply {
							//start auto-focusing every second
							setAutoCancelDuration(1, TimeUnit.SECONDS)
						}.build()
						camera.cameraControl.startFocusAndMetering(autoFocusAction)
					} catch (e: CameraInfoUnavailableException) {
						Log.d("ERROR", "cannot access camera", e)
					}
				}
			}, executor)
			previewView
		},
	)
}
