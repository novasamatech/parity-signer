package io.parity.signer.modals

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.*
import androidx.compose.material.Button
import androidx.compose.material.ButtonDefaults
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.unit.dp
import io.parity.signer.components.transactionCards.TCAuthor
import io.parity.signer.components.transactionCards.TCAuthorPlain
import io.parity.signer.models.SignerDataModel

@Composable
fun TransactionSigned(signerDataModel: SignerDataModel) {

	Column (modifier = Modifier.fillMaxSize()) {
		Image(
			bitmap = signerDataModel.getSignedQR(),
			contentDescription = "Signed transaction",
			contentScale = ContentScale.FillWidth,
			modifier = Modifier.fillMaxWidth()
		)
		Spacer(modifier = Modifier.padding(8.dp))
		when (signerDataModel.signingAuthor.optString("type")) {
			"author" -> {
				TCAuthor(
					payload = signerDataModel.signingAuthor.getJSONObject("payload"),
					signerDataModel = signerDataModel
				)
			}
			"author_plain" -> {
				TCAuthorPlain(
					payload = signerDataModel.signingAuthor.getJSONObject("payload"),
					signerDataModel = signerDataModel
				)
			}
			else -> {
				Text(signerDataModel.signingAuthor.toString())
			}
		}
	}
}
