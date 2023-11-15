package io.parity.signer.screens.scan.transaction

import android.content.res.Configuration
import androidx.activity.compose.BackHandler
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.MutableState
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.pluralStringResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.PrimaryButtonWide
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.components.base.SecondaryButtonWide
import io.parity.signer.components.qrcode.AnimatedQrKeysInfo
import io.parity.signer.components.qrcode.EmptyQrCodeProvider
import io.parity.signer.domain.Callback
import io.parity.signer.domain.UnifiiStubs
import io.parity.signer.domain.getData
import io.parity.signer.domain.submitErrorState
import io.parity.signer.screens.scan.errors.TransactionErrorEmbedded
import io.parity.signer.screens.scan.transaction.components.TransactionElementSelector
import io.parity.signer.screens.scan.transaction.components.TransactionSummaryView
import io.parity.signer.screens.scan.transaction.components.toSigningTransactionModels
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.backgroundPrimary
import io.parity.signer.uniffi.*


/**
 * Old UI screen edited to work on new screens
 */
@Composable
fun TransactionsScreenFull(
	transactions: List<MTransaction>,
	signature: MSignatureReady?,
	modifier: Modifier = Modifier,
	onBack: Callback,
	onApprove: Callback,
	onImportKeys: Callback,
) {
	val detailedTransaction = remember {
		mutableStateOf<MTransaction?>(null)
	}
	detailedTransaction.value?.let { detailTransac ->
		//detailes mode
		TransactionDetailsScreen(
			transaction = detailTransac,
			modifier = modifier,
			onBack = { detailedTransaction.value = null },
		)
	} ?: run {
		//default view
		TransactionsScreenFull(
			modifier,
			onBack,
			transactions,
			detailedTransaction,
			signature,
			onApprove,
			onImportKeys,
		)
	}
}

@Composable
internal fun TransactionsScreenFull(
	modifier: Modifier,
	onBack: Callback,
	transactions: List<MTransaction>,
	detailedTransaction: MutableState<MTransaction?>,
	signature: MSignatureReady?,
	onApprove: Callback,
	onImportKeys: Callback,
) {
	// otherwise rust state machine will stuck in transaction state and won't allow navigation to default NAVBAR action when user leaves camera.
	BackHandler(onBack = onBack)
	Column(
		modifier
			.background(MaterialTheme.colors.backgroundPrimary)
	) {
		ScreenHeader(
			title = getTransactionsTitle(
				transactionsCount = transactions.size,
				previewType = transactions.previewType,
			),
			onBack = onBack
		)
		Column(
			Modifier
				.verticalScroll(rememberScrollState())
				.weight(1f)
		) {
			transactions.forEach {
				TransactionIssues(it)
			}
			//new transaction summary
			ShowTransactionsPreview(transactions, detailedTransaction)
			signature?.let {
				QrSignatureData(it)
			}
			Spacer(modifier = Modifier.weight(1f))
			ActionButtons(
				transactions,
				onBack,
				onApprove,
				onImportKeys,
			)
		}
	}
}

@Composable
private fun getTransactionsTitle(
	previewType: TransactionPreviewType,
	transactionsCount: Int
): String {
	return when (previewType) {
		is TransactionPreviewType.AddNetwork -> stringResource(R.string.transactions_title_network)
		is TransactionPreviewType.Metadata -> stringResource(R.string.transactions_title_metadata)
		TransactionPreviewType.Transfer, TransactionPreviewType.Unknown -> pluralStringResource(
			id = R.plurals.transactions_title_other,
			transactionsCount,
			transactionsCount
		)
	}
}

@Composable
private fun ShowTransactionsPreview(
	transactions: List<MTransaction>,
	detailedTransactionPreview: MutableState<MTransaction?>
) {
	transactions.withIndex()
		.filter { it.value.shouldShowAsSummaryTransaction() }
		.toSigningTransactionModels().forEach { transactionModel ->
			TransactionSummaryView(transactionModel) { index ->
				try {
					val transaction = transactions[index]
					detailedTransactionPreview.value = transaction
				} catch (e: Exception) {
					submitErrorState("wrong index of clicked transaction - sync issue? $e")
				}
			}
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
}

@Composable
private fun QrSignatureData(signature: MSignatureReady) {
	Text(
		text = stringResource(R.string.transaction_qr_header),
		color = MaterialTheme.colors.primary,
		style = SignerTypeface.TitleS,
		modifier = Modifier.padding(horizontal = 24.dp, vertical = 14.dp),
		maxLines = 1,
	)
	AnimatedQrKeysInfo<List<List<UByte>>>(
		input = signature.signatures.map { it.getData() },
		provider = EmptyQrCodeProvider(),
		modifier = Modifier
			.fillMaxWidth(1f)
			.padding(horizontal = 24.dp)
	)
}

@Composable
private fun ActionButtons(
	transactions: List<MTransaction>,
	onBack: Callback,
	onApprove: Callback,
	onImportKeys: Callback,
) {
	when (transactions.first().ttype) {
		TransactionType.SIGN -> {
			AddLogElement()

			SecondaryButtonWide(
				label = stringResource(R.string.transaction_action_done),
				withBackground = true,
				modifier = Modifier.padding(horizontal = 24.dp, vertical = 32.dp),
				onClicked = onBack,
			)
		}
		TransactionType.DONE -> {
			SecondaryButtonWide(
				label = stringResource(R.string.transaction_action_done),
				withBackground = true,
				modifier = Modifier.padding(horizontal = 24.dp, vertical = 32.dp),
				onClicked = onBack,
			)
		}
		TransactionType.STUB -> {
			PrimaryButtonWide(
				label = stringResource(R.string.transaction_action_approve),
				modifier = Modifier
					.padding(horizontal = 24.dp)
					.padding(top = 32.dp, bottom = 8.dp),
				onClicked = { onApprove() },
			)
			SecondaryButtonWide(
				label = stringResource(R.string.transaction_action_decline),
				modifier = Modifier
					.padding(horizontal = 24.dp)
					.padding(bottom = 32.dp),
				onClicked = onBack,
			)
		}
		TransactionType.READ -> {
			SecondaryButtonWide(
				label = stringResource(R.string.transaction_action_back),
				withBackground = true,
				modifier = Modifier.padding(horizontal = 24.dp, vertical = 32.dp),
				onClicked = onBack,
			)
		}

		TransactionType.IMPORT_DERIVATIONS -> {
			PrimaryButtonWide(
				label = stringResource(R.string.transaction_action_import_keys),
				modifier = Modifier
					.padding(horizontal = 24.dp)
					.padding(top = 32.dp, bottom = 8.dp),
				onClicked = onImportKeys,
			)
			SecondaryButtonWide(
				label = stringResource(R.string.transaction_action_decline),
				modifier = Modifier
					.padding(horizontal = 24.dp)
					.padding(bottom = 32.dp),
				onClicked = onBack,
			)
		}
	}
}

@Composable
private fun AddLogElement() {
	//already signed and we show qr code in this screen now
	// , so we cannot add log there
//	val comment = remember { mutableStateOf("") }
//	Text(
//		"LOG NOTE",
//		style = MaterialTheme.typography.overline,
//		color = MaterialTheme.colors.Text400
//	)
//
//	val focusRequester = remember { FocusRequester() }
//	SingleTextInput(
//		content = comment,
//		update = { comment.value = it },
//		onDone = { },
//		focusRequester = focusRequester
//	)
//	Text(
//		"visible only on this device",
//		style = MaterialTheme.typography.subtitle1,
//		color = MaterialTheme.colors.Text400
//	)
}

@Composable
internal fun TransactionIssues(transaction: MTransaction) {
	transaction.transactionIssues().let {
		if (it.isNotEmpty()) {
			TransactionErrorEmbedded(
				errors = it,
				modifier = Modifier.padding(horizontal = 16.dp, vertical = 8.dp),
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


@Preview(
	name = "light", group = "general", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xB0FFFFFF,
)
@Preview(
	name = "dark", group = "general",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewTransactionsScreenFull() {
	SignerNewTheme {
		val transaction = UnifiiStubs.makeTransactionStubList()
		TransactionsScreenFull(transaction, null, Modifier, {}, {}, {})
	}
}
