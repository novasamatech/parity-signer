package io.parity.signer.modals

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.platform.LocalConfiguration
import io.parity.signer.components.*
import io.parity.signer.models.intoImageBitmap
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.MSignatureReady

@OptIn(ExperimentalUnsignedTypes::class)
@Composable
fun SignatureReady(
	signatureReady: MSignatureReady,
	button: (action: Action, details: String, seedPhrase: String) -> Unit
) {
	Column(
		Modifier.verticalScroll(rememberScrollState())
	) {
		TransactionPreviewField(
			cardSet = signatureReady.content,
		)
		KeyCard(identity = signatureReady.authorInfo)
		NetworkCard(network = signatureReady.networkInfo)
		Image(
			bitmap = signatureReady.signature.intoImageBitmap(),
			contentDescription = "Signed transaction",
			contentScale = ContentScale.FillWidth,
			modifier = Modifier.fillMaxWidth()
		)
		BigButton(
			text = "Done",
			action = {
				button(Action.GO_BACK, "", "")
			}
		)
	}

	val height = LocalConfiguration.current.screenHeightDp
	val width = LocalConfiguration.current.screenWidthDp
	var offset by remember { mutableStateOf(0f) }

	DisposableEffect(Unit) {
		offset = width.toFloat()
		onDispose { offset = 0f }
	}
}
