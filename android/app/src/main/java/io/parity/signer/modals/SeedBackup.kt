package io.parity.signer.modals

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Lock
import androidx.compose.runtime.*
import io.parity.signer.components.HeaderBar
import io.parity.signer.components.NetworkCard
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.getSeed
import io.parity.signer.ui.theme.Bg200
import io.parity.signer.ui.theme.Crypto400
import org.json.JSONArray

/**
 * Modal to show seed phrase. Dangerous place.
 * TODO: make sure seed phrase is cleared at all times!!!
 */
@Composable
fun SeedBackup(signerDataModel: SignerDataModel) {
	val seedName = signerDataModel.modalData.value?.optString("seed_name") ?: ""
	var seedPhrase by remember { mutableStateOf("Seed phrase is not available now") }
	var error by remember { mutableStateOf(true) }
	var derivations =
		signerDataModel.modalData.value?.optJSONArray("derivations") ?: JSONArray()

	Surface(color = Bg200, shape = MaterialTheme.shapes.large) {
		Column {
			HeaderBar("Backup", seedName)
			Text("SEED PHRASE")
			Text(seedPhrase)
			Text("DERIVED KEYS")
			LazyColumn {
				for (packIndex in 0 until derivations.length()) {
					item {
						NetworkCard(derivations.getJSONObject(packIndex))
					}
					items(
						derivations.getJSONObject(packIndex).getJSONArray("id_set").length()
					) { index ->
						derivations.getJSONObject(packIndex).getJSONArray("id_set")
								.getJSONObject(index).let {
									if (it.optString("path").isBlank()) {
										Text("seed key")
									} else {
										Row {
											Text(it.optString("path"))
											if (it.optBoolean("has_pwd")) {
												Text("///")
												Icon(Icons.Default.Lock, "Password protected", tint = Crypto400)
											}
										}
									}
							}
					}
				}
			}
		}
	}
	DisposableEffect(Unit) {
		if (signerDataModel.alertState.value == io.parity.signer.ShieldAlert.None) {
			signerDataModel.authentication.authenticate(signerDataModel.activity) {
				seedPhrase = signerDataModel.getSeed(seedName, backup = true)
				if (seedPhrase.isBlank()) {
					seedPhrase = "Seed phrase is not available now"
					error = true
				} else {
					error = false
				}
			}
		} else {
			seedPhrase == "Seed phrase is not available now"
			error = true
		}
		onDispose { seedPhrase = "" }
	}
}
