package io.parity.signer.screens

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import io.parity.signer.ButtonID
import io.parity.signer.components.KeyCard
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.getSeed
import io.parity.signer.models.pushButton
import org.json.JSONArray

@Composable
fun SignSufficientCrypto(signerDataModel: SignerDataModel) {
	val identities =
		signerDataModel.screenData.value?.optJSONArray("identities") ?: JSONArray()
	Column {
		Text("Select key for signing")
		LazyColumn {
			items(identities.length()) { index ->
				val identity = identities.getJSONObject(index)
				Row(Modifier.clickable {
					signerDataModel.authentication.authenticate(signerDataModel.activity) {
						val seedPhrase = signerDataModel.getSeed(
							identity.optString("seed_name")
						)
						if (seedPhrase.isNotBlank()) {
							signerDataModel.pushButton(
								ButtonID.GoForward,
								identity.optString("address_key"),
								seedPhrase
							)
						}
					}
				}) {
					KeyCard(identity = identity, signerDataModel = signerDataModel)
				}
			}
		}
	}
}
