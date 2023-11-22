package io.parity.signer.screens.scan.camera

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
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import com.google.accompanist.permissions.ExperimentalPermissionsApi
import com.google.accompanist.permissions.isGranted
import com.google.accompanist.permissions.rememberPermissionState
import com.google.accompanist.permissions.shouldShowRationale
import io.parity.signer.R
import io.parity.signer.domain.Callback
import io.parity.signer.domain.KeepScreenOn
import io.parity.signer.screens.scan.camera.*
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import kotlinx.coroutines.flow.filter
import kotlinx.coroutines.flow.filterNotNull
import kotlinx.coroutines.launch

@Composable
fun ScanScreen(
	onClose: Callback,
	performPayloads: suspend (String) -> Unit,
	onBananaSplit: (List<String>) -> Unit,
	onDynamicDerivations: suspend (String) -> Unit,
	onDynamicDerivationsTransactions: suspend (List<String>) -> Unit,
) {
	val viewModel: CameraViewModel = viewModel()

	val captured by viewModel.captured.collectAsStateWithLifecycle()
	val total by viewModel.total.collectAsStateWithLifecycle()

	val currentPerformPayloads by rememberUpdatedState(performPayloads)
	val currentOnBananaSplit by rememberUpdatedState(onBananaSplit)
	val currentOnDynamicDerivations by rememberUpdatedState(onDynamicDerivations)
	val currentOnDynamicDerivationsTransactions by rememberUpdatedState(
		onDynamicDerivationsTransactions
	)
	LaunchedEffect(viewModel) {
		//there can be data from last time camera was open since it's scanning during transition to a new screen
		viewModel.resetPendingTransactions()

		launch {
			viewModel.pendingTransactionPayloads
				.filter { it.isNotEmpty() }
				.collect {
					//scanned qr codes is signer transaction qr code
					currentPerformPayloads(it.joinToString(separator = ""))
					viewModel.resetPendingTransactions()
				}
		}

		launch {
			viewModel.bananaSplitPayload
				.filterNotNull()
				.filter { it.isNotEmpty() }
				.collect { qrData ->
					currentOnBananaSplit(qrData)
				}
		}

		launch {
			viewModel.dynamicDerivationPayload
				.filterNotNull()
				.filter { it.isNotEmpty() }
				.collect { qrData ->
					currentOnDynamicDerivations(qrData)
				}
		}

		launch {
			viewModel.dynamicDerivationTransactionPayload
				.filterNotNull()
				.filter { it.isNotEmpty() }
				.collect { qrData ->
					currentOnDynamicDerivationsTransactions(qrData)
				}
		}
	}

	Box(
		Modifier
			.fillMaxSize(1f)
			.background(MaterialTheme.colors.background)
	) {
		CameraViewWithPermission(viewModel)
		CameraBottomText()
		Column() {
			Spacer(
				modifier = Modifier
					.statusBarsPadding()
					.padding(top = 4.dp)
			)
			ScanHeader(onClose = onClose)

			Spacer(modifier = Modifier.weight(1f))
			val capturedCpy = captured
			if (capturedCpy != null) {
				ScanProgressBar(capturedCpy, total) { viewModel.resetScanValues() }
			}
		}
	}
	KeepScreenOn()
}

@Composable
private fun CameraBottomText() {
	Column(
		Modifier
			.fillMaxSize(1f)
			.padding(horizontal = 48.dp),
	) {
		val paddingTillEndOfCutout =
			(LocalConfiguration.current.screenHeightDp.dp / 2).plus(
				(LocalConfiguration.current.screenWidthDp.dp / 2) - ScanConstants.CLIP_SIDE_PADDING * 2
			)
		Spacer(modifier = Modifier.padding(top = paddingTillEndOfCutout))
		Spacer(modifier = Modifier.weight(0.5f))
		Text(
			text = stringResource(R.string.camera_screen_header_single),
			color = Color.White,
			style = SignerTypeface.TitleL,
			textAlign = TextAlign.Center,
			modifier = Modifier.fillMaxWidth(1f),
		)
		Spacer(modifier = Modifier.padding(bottom = 12.dp))
		Text(
			text = stringResource(R.string.camera_screen_description_single),
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
private fun CameraViewWithPermission(viewModel: CameraViewModel) {
	if (LocalInspectionMode.current) return

	val rationalShown = remember {
		mutableStateOf(false)
	}
	val cameraPermissionState =
		rememberPermissionState(android.Manifest.permission.CAMERA)
	if (cameraPermissionState.status.isGranted) {

		//camera content itself!
		CameraViewInternal(viewModel)
		TransparentCutoutLayout()

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
			ScanScreen({}, { _ -> }, { _ -> }, { _ -> }, { _ -> })
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
		Box() {
			TransparentCutoutLayout()
			CameraBottomText()
		}
	}
}
