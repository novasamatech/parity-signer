package io.parity.signer.screens

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import io.parity.signer.ButtonID
import io.parity.signer.components.SeedCard
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton
import org.json.JSONArray

/**
 * Select seed for creating derivations
 */
@Composable
fun SelectSeedForBackup(signerDataModel: SignerDataModel) {
	val content = signerDataModel.screenData.value?.optJSONArray("seedNameCards")
		?: JSONArray()
	LazyColumn {
		items(content.length()) { index ->
			val seed = content.getJSONObject(index)
			Row(Modifier.clickable {
				signerDataModel.pushButton(
					ButtonID.BackupSeed,
					details = seed.optString("seed_name")
				)
			}) {
				SeedCard(
					seedName = seed.optString("seed_name"),
					identicon = seed.optString("identicon")
				)
			}
		}
	}
}
