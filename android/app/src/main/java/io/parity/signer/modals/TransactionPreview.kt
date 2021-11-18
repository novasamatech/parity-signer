package io.parity.signer.modals

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.Button
import androidx.compose.material.ButtonDefaults
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.unit.dp
import io.parity.signer.components.TransactionCard
import io.parity.signer.components.transactionCards.TCAuthor
import io.parity.signer.components.transactionCards.TCAuthorPlain
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.acceptTransaction
import io.parity.signer.ui.theme.Crypto400
import io.parity.signer.ui.theme.Text300

@Composable
fun TransactionPreview(signerDataModel: SignerDataModel) {
	val transaction = signerDataModel.transaction.observeAsState()
	val actionable = signerDataModel.actionable.observeAsState()

	Column {
		LazyColumn(
			modifier = Modifier
				.weight(1f)
				.padding(8.dp)
				.border(
					BorderStroke(1.dp, Crypto400),
					RoundedCornerShape(8.dp)
				)
				.clip(RoundedCornerShape(8.dp))
				.padding(8.dp)
		) {
			items(transaction.value!!.length()) { item ->
				TransactionCard(
					card = transaction.value!!.getJSONObject(item),
					signerDataModel
				)
			}
		}
		if (actionable.value == true) {
			when (signerDataModel.signingAuthor.optString("type")) {
				"author" -> {
					TCAuthor(
						payload = signerDataModel.signingAuthor.getJSONObject("payload"),
						signerDataModel = signerDataModel
					)
				}
				"author_plain" -> {
					TCAuthorPlain(
						payload = signerDataModel.signingAuthor.getJSONObject("payload"),
						signerDataModel = signerDataModel
					)
				}
				else -> {
					Text(signerDataModel.signingAuthor.toString())
				}
			}
			Row(
				horizontalArrangement = Arrangement.Center,
				modifier = Modifier
					.fillMaxWidth()
					.clickable { signerDataModel.acceptTransaction() }
			) {
				Text("Accept")
			}
		} else {
			Row(
				horizontalArrangement = Arrangement.Center,
				modifier = Modifier
					.fillMaxWidth()
			) {
				Text("Action forbidden", color = Text300)
			}
		}
	}
}
