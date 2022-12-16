package io.parity.signer.screens.scan.transaction

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.runtime.Composable
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Modifier
import androidx.lifecycle.viewmodel.compose.viewModel
import io.parity.signer.components.*
import io.parity.signer.components.qrcode.AnimatedQrKeysInfo
import io.parity.signer.components.qrcode.EmptyQrCodeProvider
import io.parity.signer.models.Callback
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.getData
import io.parity.signer.screens.scan.transaction.components.TransactionPreviewField
import io.parity.signer.uniffi.*


/**
 * Old UI screen edited to work on new screens
 */
@Composable
fun TransactionScreen(
	transactions: List<MTransaction>,
	signature: MSignatureReady?,
	onBack: Callback,
	onFinish: Callback,
) {
	Column(
		Modifier.verticalScroll(rememberScrollState())
	) {
		Transactions(transactions)
		signature?.let {
			QrSignatureData(it)
		}
		ActionButtons(
			transactions,
			onBack,
			onFinish
		)
	}
}

@Composable
private fun QrSignatureData(signature: MSignatureReady){
	AnimatedQrKeysInfo<List<List<UByte>>>(
		input = signature.signatures.map { it.getData() },
		provider = EmptyQrCodeProvider(),
		modifier = Modifier.fillMaxWidth(1f)
	)
}

@Composable
private fun ActionButtons(
	transactions: List<MTransaction>,
	onBack: Callback,
	onFinish: Callback
) {
	val action = transactions.first().ttype
	when (action) {
		TransactionType.SIGN -> {

			//already signed so we show qr code, so we cannot add log there
//			val comment = remember { mutableStateOf("") }
//				Text(
//					"LOG NOTE",
//					style = MaterialTheme.typography.overline,
//					color = MaterialTheme.colors.Text400
//				)
//
//				val focusManager = LocalFocusManager.current
//				val focusRequester = remember { FocusRequester() }
//				SingleTextInput(content = comment,
//					update = { comment.value = it },
//					onDone = { },
//					focusManager = focusManager,
//					focusRequester = focusRequester
//				)
//				Text(
//					"visible only on this device",
//					style = MaterialTheme.typography.subtitle1,
//					color = MaterialTheme.colors.Text400
//				)

			val scope = rememberCoroutineScope()
			BigButton(text = "Unlock key and sign", action = {
//todo scan this should never be the case
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

@Composable
private fun Transactions(screenTransactions: List<MTransaction>) {
	for (transaction in screenTransactions) {
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
}

