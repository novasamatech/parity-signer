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
import io.parity.signer.uniffi.MscNetworkInfo
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
		when (val txCard = card.card) {
			is Card.AuthorCard -> TCAuthor(author = txCard.f)
			is Card.AuthorPlainCard -> TCAuthorPlain(author = txCard.f)
			is Card.AuthorPublicKeyCard -> TCAuthorPublicKey(key = txCard.f)
			is Card.BalanceCard -> TCBalance(currency = txCard.f)
			is Card.BitVecCard -> TCBitVec(bitVec = txCard.f)
			is Card.BlockHashCard -> TCBlockHash(text = txCard.f)
			is Card.CallCard -> TCMethod(payload = txCard.f)
			is Card.DefaultCard -> Text(
				txCard.f,
				style = MaterialTheme.typography.body2,
				color = MaterialTheme.colors.Text600
			)
			is Card.DerivationsCard -> TCDerivations(payload = txCard.f)
			is Card.EnumVariantNameCard -> TCEnumVariantName(name = txCard.f)
			Card.EraImmortalCard -> TCEraImmortal()
			is Card.EraMortalCard -> TCEra(era = txCard.f)
			is Card.ErrorCard -> TCError(error = txCard.f)
			is Card.FieldNameCard -> TCFieldName(fieldName = txCard.f)
			is Card.FieldNumberCard -> TCFieldNumber(fieldNumber = txCard.f)
			is Card.IdCard -> TCID(txCard.f)
			is Card.IdentityFieldCard -> TCIdentityField(text = txCard.f)
			is Card.MetaCard -> TCMeta(meta = txCard.f)
			is Card.NameVersionCard -> TCNameVersion(nameVersion = txCard.f)
			is Card.NetworkGenesisHashCard -> TCGenesisHash(payload = txCard.f)
			is Card.NetworkInfoCard -> NetworkCard(
				network = MscNetworkInfo(
					networkTitle = txCard.f.networkTitle,
					networkLogo = txCard.f.networkLogo
				)
			)
			is Card.NetworkNameCard -> TCNetworkName(text = txCard.f)
			is Card.NewSpecsCard -> TCNewSpecs(specs = txCard.f)
			is Card.NonceCard -> TCNonce(text = txCard.f)
			Card.NoneCard -> {}
			is Card.PalletCard -> TCPallet(text = txCard.f)
			is Card.TextCard -> Text(String(txCard.f.decodeHex()))
			is Card.TipCard -> TCTip(txCard.f)
			is Card.TipPlainCard -> TCTipPlain(txCard.f)
			is Card.TxSpecCard -> TCTXSpec(txCard.f)
			is Card.TxSpecPlainCard -> TCTXSpecPlain(txCard.f)
			is Card.TypesInfoCard -> TCTypesInfo(txCard.f)
			is Card.VarNameCard -> TCVarName(txCard.f)
			is Card.VerifierCard -> TCVerifier(txCard.f)
			is Card.WarningCard -> TCWarning(txCard.f)
		}
	}
}
