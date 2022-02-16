package io.parity.signer.modals

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.unit.dp
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.intoImageBitmap
import io.parity.signer.ui.theme.Bg000
import io.parity.signer.ui.theme.modal

@Composable
fun SignatureReady(signerDataModel: SignerDataModel) {

	Column {
		Surface(
			shape = MaterialTheme.shapes.modal,
			color = MaterialTheme.colors.Bg000
		) {
			Column(modifier = Modifier
				.fillMaxSize()
				.padding(20.dp)) {
				Text("Your signature")
				Text("Scan this into your application")
				Image(
					bitmap = signerDataModel.modalData.value?.getString("signature")!!
						.intoImageBitmap(),
					contentDescription = "Signed transaction",
					contentScale = ContentScale.FillWidth,
					modifier = Modifier.fillMaxWidth()
				)
			}
		}
	}
}
