package io.parity.signer.screens.scan

import android.content.res.Configuration
import androidx.camera.core.*
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalConfiguration
import androidx.compose.ui.platform.LocalInspectionMode
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.lifecycle.viewmodel.compose.viewModel
import com.google.accompanist.permissions.ExperimentalPermissionsApi
import com.google.accompanist.permissions.isGranted
import com.google.accompanist.permissions.rememberPermissionState
import com.google.accompanist.permissions.shouldShowRationale
import io.parity.signer.R
import io.parity.signer.components.KeepScreenOn
import io.parity.signer.models.Callback
import io.parity.signer.screens.scan.camera.*
import io.parity.signer.screens.scan.camera.CameraViewInternal
import io.parity.signer.screens.scan.camera.ScanHeader
import io.parity.signer.screens.scan.camera.TransparentClipLayout
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
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
	val isMultiscan by viewModel.isMultiscanMode.collectAsState()

	LaunchedEffect(key1 = isMultiscan) {
		if (!isMultiscan) {
			//if multi scan mode - on button click reaction should be handled
			viewModel.pendingTransactionPayloads
				.filter { it.isNotEmpty() }
				.collect {
					val transactions = viewModel.getTransactionsFromPendingPayload()
					if (transactions.isNotEmpty()) {
						//scanned qr codes is signer transaction qr code
						viewModel.resetPendingTransactions()
						onNavigateToTransaction(transactions)
					} else {
						viewModel.resetPendingTransactions()
					}
				}
		}
	}

	Box(
		Modifier
			.fillMaxSize(1f)
			.background(MaterialTheme.colors.background)
	) {
		CameraViewPermission(viewModel)
		CameraBottomText(isMultiscan)
		Column() {
			Spacer(modifier = Modifier
				.statusBarsPadding()
				.padding(top = 12.dp))
			ScanHeader(onClose = onClose)

			Spacer(modifier = Modifier.weight(1f))
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

	}
	KeepScreenOn()
}

@Composable
private fun CameraMultiModProceed(
	viewModel: CameraViewModel,
	isMultimode: State<Boolean>,
	pendingTransactions: State<Set<String>>,
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
		val paddingTop = ScanConstants.CLIP_TOP_PADDING +
		//square clip height
			LocalConfiguration.current.screenWidthDp.dp - ScanConstants.CLIP_SIDE_PADDING.times(2)
		Spacer(modifier = Modifier.padding(top = paddingTop))
		Spacer(modifier = Modifier.weight(0.5f))
		Text(
			text = stringResource(
				if (isMultimode) {
					R.string.camera_screen_header_multimode
				} else {
					R.string.camera_screen_header_single
				}
			),
			color = Color.White,
			style = SignerTypeface.TitleL,
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
			style = SignerTypeface.TitleS,
			textAlign = TextAlign.Center,
			modifier = Modifier.fillMaxWidth(1f),
		)
		Spacer(modifier = Modifier.weight(0.5f))
	}
}


@OptIn(ExperimentalPermissionsApi::class)
@Composable
private fun CameraViewPermission(viewModel: CameraViewModel) {
	if (LocalInspectionMode.current) return

	val rationalShown = remember {
		mutableStateOf(false)
	}
	val cameraPermissionState =
		rememberPermissionState(android.Manifest.permission.CAMERA)
	if (cameraPermissionState.status.isGranted) {

		//camera content itself!
		CameraViewInternal(viewModel)
		TransparentClipLayout()

	} else if (cameraPermissionState.status.shouldShowRationale
		&& !rationalShown.value
	) {
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
		Text(
			text = stringResource(R.string.camera_no_permission_text),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.BodyL,
			textAlign = TextAlign.Center,
			modifier = Modifier
				.fillMaxWidth(1f)
				.padding(top = 150.dp),
		)
		rationalShown.value = true
		LaunchedEffect(key1 = Unit) {
			launch { cameraPermissionState.launchPermissionRequest() }
		}
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
	SignerNewTheme {
		Box(modifier = Modifier.size(350.dp, 550.dp)) {
			ScanScreen({}, { _ -> })
		}
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
private fun PreviewBottomText() {
	SignerNewTheme {
		CameraBottomText(isMultimode = false)
	}
}
