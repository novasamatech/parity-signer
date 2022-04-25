package io.parity.signer.screens

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import io.parity.signer.components.KeyCard
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.getSeed
import io.parity.signer.models.pushButton
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.MSignSufficientCrypto

@Composable
fun SignSufficientCrypto(
	sc: MSignSufficientCrypto,
	signerDataModel: SignerDataModel
) {
	val identities = sc.identities
	Column {
		Text("Select key for signing")
		LazyColumn {
			items(identities.size) { index ->
				val identity = identities[index]
				Row(Modifier.clickable {
					signerDataModel.authentication.authenticate(signerDataModel.activity) {
						val seedPhrase = signerDataModel.getSeed(
							identity.seedName
						)
						if (seedPhrase.isNotBlank()) {
							signerDataModel.pushButton(
								Action.GO_FORWARD,
								identity.addressKey,
								seedPhrase
							)
						}
					}
				}) {
					/* TODO MRawKey -> Address conversion
					KeyCard(identity = identity)
					 */
				}
			}
		}
	}
}
