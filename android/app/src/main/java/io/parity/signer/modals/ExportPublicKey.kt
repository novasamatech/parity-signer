package io.parity.signer.modals

import android.widget.ImageView
import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material.Icon
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.layout.ContentScale
import io.parity.signer.models.SignerDataModel
import kotlin.math.sign

@Composable
fun ExportPublicKey(signerDataModel: SignerDataModel) {
	Column (modifier = Modifier.fillMaxSize()) {
		Image(
			bitmap = signerDataModel.exportPublicKey(),
			contentDescription = "QR with public key to scan",
			contentScale = ContentScale.FillWidth,
			modifier = Modifier.fillMaxSize()
		)
	}
}
