package io.parity.signer.screens

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.platform.LocalFocusManager
import io.parity.signer.components.*
import io.parity.signer.ui.theme.Text400
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.MTransaction
import io.parity.signer.uniffi.TransactionType

@Composable
fun TransactionPreview(
	transactions: List<MTransaction>,
	button: (action: Action, details: String, seedPhrase: String) -> Unit,
	signTransaction: (comment: String, seedName: String) -> Unit
) {
	Column(
		Modifier.verticalScroll(rememberScrollState())
	) {
		for (transaction in transactions) {
			TransactionPreviewField(
				cardSet = transaction.content,
			)
			transaction.authorInfo?.let {
				KeyCardOld(identity = it)
			}
			transaction.networkInfo?.let {
				NetworkCard(NetworkCardModel(it.networkTitle, it.networkLogo))
			}
		}
		val action = transactions.first().ttype
		val comment = remember { mutableStateOf("") }
		val focusManager = LocalFocusManager.current
		val focusRequester = remember { FocusRequester() }
		when (action) {
			TransactionType.SIGN -> {
				Text(
					"LOG NOTE",
					style = MaterialTheme.typography.overline,
					color = MaterialTheme.colors.Text400
				)

				SingleTextInput(
					content = comment,
					update = { comment.value = it },
					onDone = { },
					focusManager = focusManager,
					focusRequester = focusRequester
				)

				Text(
					"visible only on this device",
					style = MaterialTheme.typography.subtitle1,
					color = MaterialTheme.colors.Text400
				)

				BigButton(
					text = "Unlock key and sign",
					action = {
						signTransaction(
							comment.value, transactions.firstOrNull()
								?.authorInfo?.address?.seedName ?: ""
						)
					}
				)
				BigButton(
					text = "Decline",
					action = {
						button(Action.GO_BACK, "", "")
					}
				)
			}
			TransactionType.DONE ->
				BigButton(
					text = "Done",
					action = {
						button(Action.GO_BACK, "", "")
					}
				)
			TransactionType.STUB -> {
				BigButton(
					text = "Approve",
					action = {
						button(Action.GO_FORWARD, "", "")
					}
				)
				BigButton(
					text = "Decline",
					action = {
						button(Action.GO_BACK, "", "")
					}
				)
			}
			TransactionType.READ ->
				BigButton(
					text = "Back",
					action = {
						button(Action.GO_BACK, "", "")
					}
				)
			TransactionType.IMPORT_DERIVATIONS -> {
				BigButton(
					text = "Select seed",
					action = {
						button(Action.GO_FORWARD, "", "")
					}
				)
				BigButton(
					text = "Decline",
					action = {
						button(Action.GO_BACK, "", "")
					}
				)
			}
		}
	}
}

