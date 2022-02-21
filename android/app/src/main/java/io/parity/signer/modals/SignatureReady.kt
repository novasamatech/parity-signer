package io.parity.signer.modals

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.platform.LocalConfiguration
import androidx.compose.ui.unit.dp
import io.parity.signer.ButtonID
import io.parity.signer.components.BigButton
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.intoImageBitmap
import io.parity.signer.models.pushButton
import io.parity.signer.ui.theme.Bg000
import io.parity.signer.ui.theme.modal

@Composable
fun SignatureReady(signerDataModel: SignerDataModel) {
	Column(
		Modifier.verticalScroll(rememberScrollState(LocalConfiguration.current.screenHeightDp))
	) {
		Spacer(Modifier.height(LocalConfiguration.current.screenHeightDp.dp).fillMaxWidth())
		Surface(
			shape = MaterialTheme.shapes.modal,
			color = MaterialTheme.colors.Bg000,
			modifier = Modifier.height(LocalConfiguration.current.screenHeightDp.dp)
		) {
			Column(
				modifier = Modifier
					.fillMaxSize()
					.padding(20.dp)
			) {
				Text("Your signature")
				Text("Scan this into your application")
				Image(
					bitmap = signerDataModel.modalData.value?.getString("signature")!!
						.intoImageBitmap(),
					contentDescription = "Signed transaction",
					contentScale = ContentScale.FillWidth,
					modifier = Modifier.fillMaxWidth()
				)
				Spacer(Modifier.weight(1f))
				BigButton(
					text = "Done",
					action = {
						signerDataModel.pushButton(ButtonID.GoBack, "", "")
					}
				)
			}
		}
		Spacer(Modifier.height((LocalConfiguration.current.screenHeightDp - LocalConfiguration.current.screenWidthDp).dp).fillMaxWidth())
	}
}
