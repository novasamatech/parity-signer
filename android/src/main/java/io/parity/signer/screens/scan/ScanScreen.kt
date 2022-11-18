package io.parity.signer.screens.scan

import android.content.res.Configuration
import android.view.ViewGroup
import androidx.camera.core.CameraSelector
import androidx.camera.core.ImageAnalysis
import androidx.camera.lifecycle.ProcessCameraProvider
import androidx.camera.view.PreviewView
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
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
import io.parity.signer.models.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.TypefaceNew
import io.parity.signer.uniffi.MTransaction
import kotlinx.coroutines.flow.filter
import kotlinx.coroutines.launch

@Composable
fun ScanScreen(
	onClose: Callback,
	onNavigateToTransaction: (List<MTransaction>) -> Unit,
) {
	val viewModel: CameraViewModel = viewModel()

	val captured by viewModel.captured.collectAsState()
	val total by viewModel.total.collectAsState()
	val isMultimode by viewModel.isMultiscanMode.collectAsState()

	LaunchedEffect(key1 = isMultimode) {
		if (!isMultimode) {
			//if multimode - on button click reaction should be handled
			viewModel.pendingTransactionPayloads
				.filter { it.isNotEmpty() }
				.collect {
					val transactions = viewModel.getTransactionsFromPendingPayload()
					onNavigateToTransaction(transactions)
				}
		}
	}

	Box(
		Modifier
			.fillMaxSize(1f)
			.background(MaterialTheme.colors.background)
	) {
		CameraViewPermission(viewModel)
		ScanHeader(Modifier.statusBarsPadding(), onClose)
		CameraBottomText(isMultimode)
		val capturedCpy = captured
		if (capturedCpy != null) {
			ScanProgressBar(capturedCpy, total) { viewModel.resetScanValues() }
		} else {
			CameraMultiModProceed(
				viewModel = viewModel,
				isMultimode = viewModel.isMultiscanMode.collectAsState(),
				pendingTransactions = viewModel.pendingTransactionPayloads.collectAsState(),
				onNavigateToTransaction = onNavigateToTransaction,
			)
		}
	}
	KeepScreenOn()
}

@Composable
private fun CameraMultiModProceed(
	viewModel: CameraViewModel,
	isMultimode: State<Boolean>,
	pendingTransactions: State<List<String>>,
	onNavigateToTransaction: (List<MTransaction>) -> Unit,
) {
	//todo multisign for multimode implement UI
//	if (isMultimode.value && pendingTransactions.value.isNotEmpty()) {
//		val coroutineScope = rememberCoroutineScope()
//		Button(onClick = {
//			coroutineScope.launch {
//				val transactions = viewModel.getTransactionsFromPendingPayload()
//				onNavigateToTransaction(transactions)
//			}
//		}) {
//			Text(text = "some")
//		}
//	}
}

@Composable
private fun CameraBottomText(isMultimode: Boolean) {
	Column(
		Modifier
			.fillMaxSize(1f)
			.padding(horizontal = 48.dp),
	) {
		Spacer(modifier = Modifier.weight(1f))
		Text(
			text = stringResource(
				if (isMultimode) {
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
				if (isMultimode) {
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


@OptIn(ExperimentalPermissionsApi::class)
@Composable
private fun CameraViewPermission(viewModel: CameraViewModel) {
	if (LocalInspectionMode.current) return

	val rationalShown = remember {
		mutableStateOf(false)
	}
	//todo dmitry test it properly
	val cameraPermissionState =
		rememberPermissionState(android.Manifest.permission.CAMERA)
	if (cameraPermissionState.status.isGranted) {
		CameraViewInternal(viewModel)
		TransparentClipLayout()
	} else if (cameraPermissionState.status.shouldShowRationale
		&& !rationalShown.value) {
		AlertDialog(
			onDismissRequest = {
				rationalShown.value = true
			},
			confirmButton = {
				val scope = rememberCoroutineScope()
				Button(
					colors = ButtonDefaults.buttonColors(backgroundColor = MaterialTheme.colors.background),
					onClick = {
						scope.launch { cameraPermissionState.launchPermissionRequest() }
						rationalShown.value = true
					},
				) {
					Text(text = stringResource(R.string.generic_ok))
				}
			},
			title = { Text(text = stringResource(R.string.camera_jastification_title)) },
			text = { Text(text = stringResource(R.string.camera_jastification_message)) },
		)
	} else {
		Text(stringResource(R.string.camera_no_permission_text))
		LaunchedEffect(key1 = Unit) {
			launch { cameraPermissionState.launchPermissionRequest() }
		}
	}
}

@Composable
private fun CameraViewInternal(viewModel: CameraViewModel) {
	val lifecycleOwner = LocalLifecycleOwner.current
	val context = LocalContext.current
	val cameraProviderFuture =
		remember { ProcessCameraProvider.getInstance(context) }
	val coroutineScope = rememberCoroutineScope()

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
				val camera = cameraProvider.bindToLifecycle(
					lifecycleOwner,
					cameraSelector,
					imageAnalysis,
					preview
				)
				if (camera.cameraInfo.hasFlashUnit()) {
					coroutineScope.launch {
						viewModel.isTorchEnabled.collect {
							camera.cameraControl.enableTorch(it)
						}
					}
				}
			}, executor)
			previewView
		},
	)
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
	SignerNewTheme {
		Box(modifier = Modifier.size(350.dp, 550.dp)) {
			ScanScreen({}, { _ -> })
		}
	}
}
