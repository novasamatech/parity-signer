package io.parity.signer.screens.scan.old

import androidx.compose.foundation.gestures.Orientation
import androidx.compose.foundation.gestures.draggable
import androidx.compose.foundation.gestures.rememberDraggableState
import androidx.compose.foundation.layout.*
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalConfiguration
import androidx.compose.ui.unit.IntOffset
import androidx.compose.ui.unit.dp
import io.parity.signer.components.BigButton
import io.parity.signer.components.qrcode.AnimatedQrKeysInfo
import io.parity.signer.components.qrcode.EmptyQrCodeProvider
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.getData
import io.parity.signer.models.navigate
import io.parity.signer.ui.theme.Bg000
import io.parity.signer.ui.theme.modal
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.MSignatureReady

@Composable
fun SignatureReady(
	signatureReady: MSignatureReady,
	signerDataModel: SignerDataModel
) {
	val height = LocalConfiguration.current.screenHeightDp
	val width = LocalConfiguration.current.screenWidthDp
	var offset by remember { mutableStateOf(0f) }
	Surface(
		shape = MaterialTheme.shapes.modal,
		color = MaterialTheme.colors.Bg000,
		modifier = Modifier
			.height(height.dp)
			.offset { IntOffset(0, offset.toInt()) }
			.draggable(
				orientation = Orientation.Vertical,
				state = rememberDraggableState { delta ->
					offset += delta
					if (offset < 0) offset = 0f
					//if (offset > ) offset = height.toFloat()
				},
			)

	) {
		Column(
			modifier = Modifier
				.fillMaxSize()
				.padding(20.dp)
		) {
			Text("Your signature")
			Text("Scan this into your application")
			AnimatedQrKeysInfo<List<List<UByte>>>(
				input = signatureReady.signatures.map { it.getData() },
				provider = EmptyQrCodeProvider(),
				modifier = Modifier.fillMaxWidth(1f)
			)
			Spacer(Modifier.weight(1f))
			BigButton(
				text = "Done",
				action = {
					signerDataModel.navigate(Action.GO_BACK, "", "")
				}
			)
		}
	}
	DisposableEffect(Unit) {
		offset = width.toFloat()
		onDispose { offset = 0f }
	}
}
