package io.parity.signer.components.TransactionCards

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import io.parity.signer.models.SignerDataModel
import io.parity.signer.ui.theme.Typography

@Composable
fun TCID(id: String, signerDataModel: SignerDataModel) {
	Row {
		Image(signerDataModel.getIdenticon(id, 80), "identicon")
		Column {
			Text(id)
			//Text(identity.get("public_key").toString(), style = Typography.body2)
		}
	}
}
