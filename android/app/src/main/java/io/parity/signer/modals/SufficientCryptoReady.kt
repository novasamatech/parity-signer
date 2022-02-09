package io.parity.signer.modals

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.*
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.unit.dp
import io.parity.signer.components.HeaderBar
import io.parity.signer.components.KeyCard
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.intoImageBitmap
import org.json.JSONObject

@Composable
fun SufficientCryptoReady(signerDataModel: SignerDataModel) {
	Surface() {
		Column(modifier = Modifier
			.fillMaxSize()
			.padding(20.dp)) {
			HeaderBar("Your signature", "Scan this into your application")
			Image(
				bitmap = signerDataModel.modalData.value?.getString("sufficient")!!
					.intoImageBitmap(),
				contentDescription = "Signed update",
				contentScale = ContentScale.FillWidth,
				modifier = Modifier.fillMaxWidth()
			)
			KeyCard(
				identity = signerDataModel.modalData.value?.optJSONObject("author_info")
					?: JSONObject()
			)
			Text(
				"Payload: " + signerDataModel.modalData.value?.optJSONObject("content")
					?.optString("type")
			)
		}
	}
}
