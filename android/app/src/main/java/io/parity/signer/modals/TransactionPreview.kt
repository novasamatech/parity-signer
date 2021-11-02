package io.parity.signer.modals

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.Button
import androidx.compose.material.ButtonDefaults
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import io.parity.signer.components.TransactionCard
import io.parity.signer.models.SignerDataModel

@Composable
fun TransactionPreview(signerDataModel: SignerDataModel) {
	val transaction = signerDataModel.transaction.observeAsState()
	val actionable = signerDataModel.actionable.observeAsState()

	Column {
		LazyColumn (modifier = Modifier.weight(1f)) {
			items(transaction.value!!.length()) { item ->
				TransactionCard(card = transaction.value!!.getJSONObject(item), signerDataModel)
			}
		}
		Row(
			horizontalArrangement = Arrangement.Center,
			modifier = Modifier.fillMaxWidth().clickable { signerDataModel.acceptTransaction() }
		) {
			Text("Accept")
		}
	}
}
