package io.parity.signer.screens.scan.transaction.components

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.NetworkCard
import io.parity.signer.components.NetworkCardModel
import io.parity.signer.models.decodeHex
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
            .padding(start = (card.indent.toInt() * 10).dp)
            .fillMaxWidth()
	) {
		when (val txCard = card.card) {
			// Author cards with identicon and variable description
			is Card.AuthorPlainCard -> TCAuthorPlain(author = txCard.f) // Not present on new designs
			is Card.AuthorPublicKeyCard -> TCAuthorPublicKey(key = txCard.f)  // Not present on new designs

			// Foldable Markdown values on tap
			is Card.CallCard -> TCMethod(payload = txCard.f)  // This is used to present `Method` and provides details on tap
			is Card.EnumVariantNameCard -> TCEnumVariantName(name = txCard.f)
			is Card.FieldNameCard -> TCFieldName(fieldName = txCard.f) // Presents `dest` or `value` indentent values
			is Card.FieldNumberCard -> TCFieldNumber(fieldNumber = txCard.f)

			// Sections
			is Card.NewSpecsCard -> TCNewSpecs(specs = txCard.f) // User when adding new network, redesigned
			is Card.MetaCard -> TCMeta(meta = txCard.f) // Used when scanning metadata update, redesigned
			is Card.VerifierCard -> TCVerifier(txCard.f) // Used in metadata update, adding new network, redesigned
			is Card.DerivationsCard -> TCDerivations(payload = txCard.f)  // Not present on new designs
			is Card.TxSpecPlainCard -> TCTXSpecPlain(txCard.f) // Unknown network information for given transaction, not present on new designs

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
			is Card.TextCard -> Text(String(txCard.f.decodeHex())) // Markdown text field, not present on new designs


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
			//todo redesign
			is Card.EraMortalCard -> TCEra(era = txCard.f)
			is Card.IdCard -> TCID(txCard.f) // ID card, new designs present it without identicon
			is Card.IdentityFieldCard -> TCIdentityField(text = txCard.f)
			is Card.NameVersionCard -> TCNameVersion(nameVersion = txCard.f)
			is Card.NetworkGenesisHashCard -> TCGenesisHash(payload = txCard.f)

			is Card.NetworkNameCard -> TCNetworkName(text = txCard.f)
			is Card.NonceCard -> TCNonce(text = txCard.f)
			Card.NoneCard -> Text("None")
			is Card.PalletCard -> TCPallet(text = txCard.f)
			is Card.TipCard -> TCTip(txCard.f)
			is Card.TipPlainCard -> TCTipPlain(txCard.f)
			is Card.TxSpecCard -> TCTXSpec(txCard.f)
			is Card.VarNameCard -> TCVarName(txCard.f)
		}
	}
}
