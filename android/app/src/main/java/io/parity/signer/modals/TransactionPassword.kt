package io.parity.signer.modals

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.Text
import androidx.compose.material.TextField
import androidx.compose.runtime.*
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardCapitalization
import androidx.compose.ui.text.input.KeyboardType
import io.parity.signer.components.TransactionCard
import io.parity.signer.components.transactionCards.TCAuthor
import io.parity.signer.models.SignerDataModel

@Composable
fun TransactionPassword(signerDataModel: SignerDataModel) {
	var password by remember { mutableStateOf("") }
	val lastError = signerDataModel.lastError.observeAsState()

	Column {
		TCAuthor(payload = signerDataModel.signingAuthor, signerDataModel = signerDataModel)
		Text(lastError.value.toString())
		Text("Enter password")
		TextField(
			value = password,
			onValueChange = {
				password = it
				signerDataModel.clearError()
			},
			label = { Text("Password") },
			singleLine = true,
			keyboardOptions = KeyboardOptions(
				autoCorrect = false,
				capitalization = KeyboardCapitalization.None,
				keyboardType = KeyboardType.Password,
				imeAction = ImeAction.Done
			),
			keyboardActions = KeyboardActions(
				onDone = {  }
			)
		)
	}
}
