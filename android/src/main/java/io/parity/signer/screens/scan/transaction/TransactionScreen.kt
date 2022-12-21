package io.parity.signer.screens.scan.transaction

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.KeyCardOld
import io.parity.signer.components.NetworkCard
import io.parity.signer.components.NetworkCardModel
import io.parity.signer.components.base.PrimaryButtonWide
import io.parity.signer.components.base.SecondaryButtonWide
import io.parity.signer.components.qrcode.AnimatedQrKeysInfo
import io.parity.signer.components.qrcode.EmptyQrCodeProvider
import io.parity.signer.models.Callback
import io.parity.signer.models.getData
import io.parity.signer.screens.scan.transaction.components.TransactionPreviewField
import io.parity.signer.uniffi.MSignatureReady
import io.parity.signer.uniffi.MTransaction
import io.parity.signer.uniffi.TransactionType


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
private fun QrSignatureData(signature: MSignatureReady) {
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

			//already signed and we show qr code in this screen now
			// , so we cannot add log there
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

			SecondaryButtonWide(
				label = stringResource(R.string.transaction_action_done),
				withBackground = true,
				modifier = Modifier.padding(horizontal = 24.dp),
				onClicked = onBack,
			)
		}
		TransactionType.DONE -> {
			SecondaryButtonWide(
				label = stringResource(R.string.transaction_action_done),
				withBackground = true,
				modifier = Modifier.padding(horizontal = 24.dp),
				onClicked = onBack,
			)
		}
		TransactionType.STUB -> {
			PrimaryButtonWide(
				label = stringResource(R.string.transaction_action_approve),
				modifier = Modifier.padding(horizontal = 24.dp),
				onClicked = onFinish,
			)
			Spacer(modifier = Modifier.padding(top = 8.dp))
			SecondaryButtonWide(
				label = stringResource(R.string.transaction_action_decline),
				modifier = Modifier.padding(horizontal = 24.dp),
				onClicked = onBack,
			)
		}
		TransactionType.READ -> {
			SecondaryButtonWide(
				label = stringResource(R.string.transaction_action_back),
				withBackground = true,
				modifier = Modifier.padding(horizontal = 24.dp),
				onClicked = onBack,
			)
		}
		TransactionType.IMPORT_DERIVATIONS -> {
			PrimaryButtonWide(
				label = stringResource(R.string.transaction_action_select_seed),
				modifier = Modifier.padding(horizontal = 24.dp),
				onClicked = onFinish,
			)
			Spacer(modifier = Modifier.padding(top = 8.dp))
			SecondaryButtonWide(
				label = stringResource(R.string.transaction_action_decline),
				modifier = Modifier.padding(horizontal = 24.dp),
				onClicked = onBack,
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


private const val TAG = "Transactions screen"
