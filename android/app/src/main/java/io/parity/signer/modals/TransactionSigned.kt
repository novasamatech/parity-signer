package io.parity.signer.modals

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material.Button
import androidx.compose.material.ButtonDefaults
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.layout.ContentScale
import io.parity.signer.models.SignerDataModel

@Composable
fun TransactionSigned(signerDataModel: SignerDataModel) {

	Column (modifier = Modifier.fillMaxSize()) {
		Image(
			bitmap = signerDataModel.getSignedQR(),
			contentDescription = "Signed transaction",
			contentScale = ContentScale.FillWidth,
			modifier = Modifier.fillMaxSize()
		)
		Button(
			colors = ButtonDefaults.buttonColors(
				backgroundColor = MaterialTheme.colors.secondary,
				contentColor = MaterialTheme.colors.onSecondary,
			),
			onClick = { signerDataModel.totalRefresh() }
		) { Text("Done") }
	}
}
