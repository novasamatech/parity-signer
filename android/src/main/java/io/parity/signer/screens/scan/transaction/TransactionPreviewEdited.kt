package io.parity.signer.screens.scan.transaction

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.platform.LocalFocusManager
import io.parity.signer.components.*
import io.parity.signer.models.Callback
import io.parity.signer.models.SignResult
import io.parity.signer.ui.theme.Text400
import io.parity.signer.uniffi.MTransaction
import io.parity.signer.uniffi.ScreenData
import io.parity.signer.uniffi.TransactionType
import kotlinx.coroutines.launch

/**
 * Old UI screen edited to work on new screens
 */
@Composable
fun TransactionPreviewEdited(
	transactions: List<MTransaction>,
	onBack: Callback,
	onFinish: Callback,
	onSuccess: (List<MTransaction>) -> Unit,
	signTransaction: suspend (comment: String, seedNames: List<String>) -> SignResult
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

				SingleTextInput(content = comment,
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
				val scope = rememberCoroutineScope()
				BigButton(text = "Unlock key and sign", action = {
					scope.launch {
						val result = signTransaction(comment.value,
							transactions.mapNotNull { it.authorInfo?.address?.seedName })
						//todo dmitry handle non happy cases as well and probably in viewmodel not here
						if (result is SignResult.Success) {
							(result.navResult.screenData as? ScreenData.Transaction)?.f?.let {
									transactions ->
								onSuccess(transactions)
							}
						}
					}
				})
				BigButton(
					text = "Decline", action = onBack
				)
			}
			TransactionType.DONE -> BigButton(
				text = "Done", action = onBack
			)
			TransactionType.STUB -> {
				BigButton(
					text = "Approve", action = onFinish
				)
				BigButton(
					text = "Decline", action = onBack
				)
			}
			TransactionType.READ -> BigButton(
				text = "Back", action = onBack
			)
			TransactionType.IMPORT_DERIVATIONS -> {
				BigButton(
					text = "Select seed", action = onFinish
				)
				BigButton(
					text = "Decline", action = onBack
				)
			}
		}
	}
}

