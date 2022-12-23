package io.parity.signer.screens.scan.transaction

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.PrimaryButtonWide
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.components.base.SecondaryButtonWide
import io.parity.signer.components.qrcode.AnimatedQrKeysInfo
import io.parity.signer.components.qrcode.EmptyQrCodeProvider
import io.parity.signer.models.Callback
import io.parity.signer.models.getData
import io.parity.signer.models.sortedValueCards
import io.parity.signer.models.transactionIssues
import io.parity.signer.screens.scan.elements.TransactionErrors
import io.parity.signer.screens.scan.transaction.components.TransactionElementSelector
import io.parity.signer.screens.scan.transaction.components.TransactionSummaryView
import io.parity.signer.screens.scan.transaction.components.toSigningTransactionModels
import io.parity.signer.uniffi.MSignatureReady
import io.parity.signer.uniffi.MTransaction
import io.parity.signer.uniffi.TransactionType


/**
 * Old UI screen edited to work on new screens
 */
@Composable
fun TransactionsScreen(
	transactions: List<MTransaction>,
	title: String,
	signature: MSignatureReady?,
	onBack: Callback,
	onFinish: Callback,
) {
	Column(Modifier.fillMaxWidth()) {
		ScreenHeader(title = title, onBack = onBack)
		Column(
			Modifier.verticalScroll(rememberScrollState())
		) {
			//new transaction summary
			transactions.filter { it.shouldShowAsSummaryTransaction() }
				.toSigningTransactionModels().forEach {
				TransactionSummaryView(it) {}//todo scan on click
					//todo scan ios/NativeSigner/Screens/Scan/TransactionPreview.swift:51 show details here
			}
			//old separate transactions
			transactions.filter { !it.shouldShowAsSummaryTransaction() }.forEach {
				Column(verticalArrangement = Arrangement.spacedBy(16.dp)) {
					it.sortedValueCards.forEach {
						TransactionElementSelector(it)
					}
				}
//	transaction.authorInfo?.let {
//		KeyCardOld(identity = it)
//	}
//	transaction.networkInfo?.let {
//		NetworkCard(NetworkCardModel(it.networkTitle, it.networkLogo))
//	}
			}
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
private fun TransactionIssues(transaction: MTransaction) {
	transaction.transactionIssues().let {
		if (it.isNotEmpty()) {
			TransactionErrors(
				errors = it,
				modifier = Modifier.padding(horizontal = 16.dp),
			)
		}
	}
}

private fun MTransaction.shouldShowAsSummaryTransaction(): Boolean {
	return when (ttype) {
		// Rounded corner summary card like new transaction to send tokens
		TransactionType.SIGN,
		TransactionType.READ -> {
			true
		}
		// Used when new network is being added
		// User when network metadata is being added
		TransactionType.STUB,
		TransactionType.DONE,
		TransactionType.IMPORT_DERIVATIONS -> {
			return false
		}
	}
}
