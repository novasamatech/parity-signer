package io.parity.signer.components

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.components.transactionCards.*
import io.parity.signer.models.decodeHex
import io.parity.signer.ui.theme.Text600
import io.parity.signer.uniffi.Card
import io.parity.signer.uniffi.TransactionCard

/**
 * Selector for transaction card appearance
 */
@Composable
fun TransactionCard(card: TransactionCard) {
	Box(
		modifier = Modifier
			.padding(start = (card.indent.toInt() * 10).dp)
			.fillMaxWidth()
	) {
		val card = card.card
		when (card) {
			is Card.AuthorCard -> TCAuthor(author = card.f)
			is Card.AuthorPlainCard -> TCAuthorPlain(author = card.f)
			is Card.AuthorPublicKeyCard -> TCAuthorPublicKey(key = card.f)
			is Card.BalanceCard -> TCBalance(currency = card.f)
			is Card.BitVecCard -> TCBitVec(bitVec = card.f)
			is Card.BlockHashCard -> TCBlockHash(text = card.f)
			is Card.CallCard -> TCMethod(payload = card.f)
			is Card.DefaultCard -> Text(
				card.f,
				style = MaterialTheme.typography.body2,
				color = MaterialTheme.colors.Text600
			)
			is Card.DerivationsCard -> TCDerivations(payload = card.f)
			is Card.EnumVariantNameCard -> TCEnumVariantName(name = card.f)
			Card.EraImmortalCard -> TCEraImmortal()
			is Card.EraMortalCard -> TCEra(era = card.f)
			is Card.ErrorCard -> TCError(error = card.f)
			is Card.FieldNameCard -> TCFieldName(fieldName = card.f)
			is Card.FieldNumberCard -> TCFieldNumber(fieldNumber = card.f)
			is Card.IdCard -> TCID(card.f)
			is Card.IdentityFieldCard -> TCIdentityField(text = card.f)
			is Card.MetaCard -> TCMeta(meta = card.f)
			is Card.NameVersionCard -> TCNameVersion(nameVersion = card.f)
			is Card.NetworkGenesisHashCard -> TCGenesisHash(payload = card.f)
			is Card.NetworkInfoCard -> TODO() //NetworkCard
			is Card.NetworkNameCard -> TCNetworkName(text = card.f)
			is Card.NewSpecsCard -> TCNewSpecs(specs = card.f)
			is Card.NonceCard -> TCNonce(text = card.f)
			Card.NoneCard -> {}
			is Card.PalletCard -> TCPallet(text = card.f)
			is Card.TextCard -> Text(String(card.f.decodeHex()))
			is Card.TipCard -> TCTip(card.f)
			is Card.TipPlainCard -> TCTipPlain(card.f)
			is Card.TxSpecCard -> TCTXSpec(card.f)
			is Card.TxSpecPlainCard -> TCTXSpecPlain(card.f)
			is Card.TypesInfoCard -> TCTypesInfo(card.f)
			is Card.VarNameCard -> TCVarName(card.f)
			is Card.VerifierCard -> TCVerifier(card.f)
			is Card.WarningCard -> TCWarning(card.f)
		}
	}
}
