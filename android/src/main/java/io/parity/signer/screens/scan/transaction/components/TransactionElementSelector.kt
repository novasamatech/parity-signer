package io.parity.signer.screens.scan.transaction.components

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyListScope
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.NetworkCard
import io.parity.signer.components.NetworkCardModel
import io.parity.signer.components.base.MarkdownText
import io.parity.signer.components.base.toRichTextStr
import io.parity.signer.screens.scan.transaction.transactionElements.*
import io.parity.signer.uniffi.Card
import io.parity.signer.uniffi.TransactionCard

/**
 * Selector for transaction card appearance
 */
@Composable
fun TransactionElementSelector(card: TransactionCard) {
	Box(
		modifier = Modifier
			.padding(
				start = (card.indent.toInt() * 10).dp + 16.dp,
				end = 16.dp
			)
			.fillMaxWidth()
	) {
		when (val txCard = card.card) {
			// Author cards with identicon and variable description
			is Card.AuthorPlainCard -> TCAuthorPlain(author = txCard.f) // Not present on new designs
			is Card.AuthorPublicKeyCard -> TCAuthorPublicKey(key = txCard.f)  // Not present on new designs

			// Foldable Markdown values on tap
			is Card.CallCard -> TCValueWithToogleDocs(payload = txCard.f.toTransactionCallModel())  // This is used to present `Method` and provides details on tap
			is Card.EnumVariantNameCard -> TCValueWithToogleDocs(payload = txCard.f.toTransactionCallModel())
			is Card.FieldNameCard -> TCValueWithMarkdownTrio(value = txCard.f.toTCFieldNameModel()) // Presents `dest` or `value` indentent values
			is Card.FieldNumberCard -> TCValueWithMarkdownTrio(value = txCard.f.toTCFieldNameModel())

			// Sections
			is Card.NewSpecsCard -> TCAddNetwork(specs = txCard.f) // User when adding new network, redesigned
			is Card.MetaCard -> TCMeta(meta = txCard.f.toTransactionMetadataModel()) // Used when scanning metadata update, redesigned
			is Card.VerifierCard -> TCVerifier(txCard.f) // Used in metadata update, adding new network, redesigned
			is Card.DerivationsCard -> TCDerivations(payload = txCard.f.map { it.name })  // Not present on new designs
			is Card.TxSpecPlainCard -> TCUnknownNetwork(txCard.f) // Unknown network information for given transaction, not present on new designs

			// Error handling
			is Card.ErrorCard -> TCError(error = txCard.f)
			is Card.WarningCard -> TCWarning(txCard.f)

			// Simple values with identicons / icons / markdown
			is Card.NetworkInfoCard -> NetworkCard( // Not present in new designs
				network = NetworkCardModel(
					networkTitle = txCard.f.networkTitle,
					networkLogo = txCard.f.networkLogo
				)
			)
			is Card.TypesInfoCard -> TCTypesInfo(txCard.f) // Not present in new designs
			is Card.TextCard -> MarkdownText(txCard.f.toRichTextStr()) // Markdown text field, not present on new designs

			// Simple values - redesigned
			is Card.AuthorCard -> TCNameValueElement(
				name = stringResource(R.string.transaction_field_from),
				value = txCard.f.base58,
				valueInSameLine = false
			)
			is Card.BalanceCard -> TCNameValueElement(
				value = "${txCard.f.amount} ${txCard.f.units}",
			)
			is Card.BitVecCard -> TCNameValueElement(
				name = stringResource(R.string.transaction_field_bitvec),
				value = txCard.f,
			)
			is Card.BlockHashCard -> TCNameValueElement(
				name = stringResource(R.string.transaction_field_backhash),
				value = txCard.f,
				valueInSameLine = false
			)
			is Card.DefaultCard -> TCNameValueElement(
				name = txCard.f,
			)
			Card.EraImmortalCard -> TCNameValueElement(
				name = stringResource(R.string.transaction_field_transaction_immortal),
			)
			is Card.EraMortalCard -> TCEraMortal(era = txCard.f)
			is Card.IdCard -> TCID(txCard.f.base58) // ID card, new designs present it without identicon
			is Card.IdentityFieldCard -> TCNameValueElement(
				name = stringResource(R.string.transaction_field_identityfield),
				value = txCard.f,
			)
			is Card.NameVersionCard -> TCNameValueElement(
				name = txCard.f.name,
				value = txCard.f.version,
			)
			is Card.NetworkGenesisHashCard -> TCNameValueElement(
				name = stringResource(R.string.transaction_field_genesis_hash),
				value = txCard.f,
				valueInSameLine = false,
			)
			is Card.NetworkNameCard -> TCNameValueElement(
				name = stringResource(R.string.transaction_field_network_name),
				value = txCard.f,
			)
			is Card.NonceCard -> TCNameValueElement(
				name = stringResource(R.string.transaction_field_nonce),
				value = txCard.f,
			)
			Card.NoneCard -> TCNameValueElement(
				name = stringResource(R.string.transaction_field_none),
			)
			is Card.PalletCard -> TCNameValueElement(
				name = stringResource(R.string.transaction_field_pallet),
				value = txCard.f,
			)
			is Card.TipCard -> TCNameValueElement(
				name = stringResource(R.string.transaction_field_tip),
				value = "${txCard.f.amount} ${txCard.f.units}",
			)
			is Card.TipPlainCard -> TCNameValueElement(
				name = stringResource(R.string.transaction_field_tip),
				value = txCard.f,
			)
			is Card.TxSpecCard -> TCNameValueElement(
				name = stringResource(R.string.transaction_field_tx_version),
				value = txCard.f,
			)
			is Card.VarNameCard -> TCNameValueElement(
				value = txCard.f,
			)
		}
	}
}

@Composable
fun TransactionCards(
	transactions: List<TransactionCard>,
) {
	transactions.forEach {
		TransactionElementSelector(it)
	}
}
