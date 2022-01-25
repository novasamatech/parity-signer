package io.parity.signer.modals

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.unit.dp
import io.parity.signer.components.transactionCards.TCAuthor
import io.parity.signer.components.transactionCards.TCAuthorPlain
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.intoImageBitmap

@Composable
fun SignatureReady(signerDataModel: SignerDataModel) {

	Surface() {
		Column (modifier = Modifier.fillMaxSize()) {
			Spacer(modifier = Modifier.padding(8.dp))
			Text("Your signature")
			Text("Scan this into your application")
			Image(
				bitmap = signerDataModel.modalData.value?.getString("signature")!!.intoImageBitmap(),
				contentDescription = "Signed transaction",
				contentScale = ContentScale.FillWidth,
				modifier = Modifier.fillMaxWidth()
			)

		}
	}
}
