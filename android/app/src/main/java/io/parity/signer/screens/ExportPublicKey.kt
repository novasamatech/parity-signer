package io.parity.signer.screens

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.unit.dp
import io.parity.signer.components.HeaderBar
import io.parity.signer.components.KeyCard
import io.parity.signer.components.NetworkCard
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.intoImageBitmap
import org.json.JSONObject

@Composable
fun ExportPublicKey(signerDataModel: SignerDataModel) {
	val address = signerDataModel.screenData.value ?: JSONObject()

	Column(
		modifier = Modifier
			.fillMaxWidth()
			.verticalScroll(rememberScrollState())
	) {
		KeyCard(identity = address)
		NetworkCard(address)
		Image(
			address.optString("qr").intoImageBitmap(),
			contentDescription = "QR with address to scan",
			contentScale = ContentScale.FillWidth,
			modifier = Modifier.padding(12.dp).fillMaxWidth(1f)
		)
		HeaderBar(line1 = "KEY DETAILS", line2 = "")
		Row {
			Text("Base58 key:")
			Text(address.optString("base58"))
		}
		Row {
			Text("Hex key:")
			Text(address.optString("pubkey"))
		}
		Row {
			Text("Seed name:")
			Text(address.optString("seed_name"))
		}
	}
}
