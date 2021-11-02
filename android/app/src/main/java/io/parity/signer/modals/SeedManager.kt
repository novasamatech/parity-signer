package io.parity.signer.modals

import androidx.compose.foundation.Image
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.Button
import androidx.compose.material.Icon
import androidx.compose.material.IconButton
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Delete
import androidx.compose.material.icons.filled.List
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import io.parity.signer.KeyManagerModal
import io.parity.signer.components.SeedCard
import io.parity.signer.models.SignerDataModel

@Composable
fun SeedManager(signerDataModel: SignerDataModel) {
	val seedNames = signerDataModel.seedNames.observeAsState()

	LazyColumn {
		//keys should be defined already, can't panic
		items(seedNames.value!!.size) { item ->
			Row(Modifier.fillMaxWidth()) {
				Row(Modifier.clickable {
					signerDataModel.selectSeed(seedNames.value!![item])
					signerDataModel.clearKeyManagerScreen()
				}
					.weight(1f, true)
				) {
					SeedCard(
						seedName = seedNames.value!![item],
						signerDataModel = signerDataModel
					)
				}
				IconButton(onClick = {
					signerDataModel.selectSeed(seedNames.value!![item])
				}) {
					Icon(Icons.Default.List, contentDescription = "Backup seed")
				}
				IconButton(onClick = {

				}) {
					Icon(Icons.Default.Delete, contentDescription = "Romove seed")
				}
			}
		}
	}
}
