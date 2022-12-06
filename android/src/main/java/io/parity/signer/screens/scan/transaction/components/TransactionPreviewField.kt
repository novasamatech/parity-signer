package io.parity.signer.screens.scan.transaction.components

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.LazyListScope
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.Crypto400
import io.parity.signer.uniffi.TransactionCard
import io.parity.signer.uniffi.TransactionCardSet

@Composable
fun TransactionPreviewField(
	cardSet: TransactionCardSet,
) {
	LazyColumn(
		modifier = Modifier
            .aspectRatio(1f)
            .padding(8.dp)
            .border(
                BorderStroke(1.dp, MaterialTheme.colors.Crypto400),
                RoundedCornerShape(8.dp)
            )
            .clip(RoundedCornerShape(8.dp))
            .padding(8.dp)
	) {
		transactionCards(cardSet.author)
		transactionCards(cardSet.error)
		transactionCards(cardSet.extensions)
		transactionCards(cardSet.importingDerivations)
		transactionCards(cardSet.message)
		transactionCards(cardSet.meta)
		transactionCards(cardSet.method)
		transactionCards(cardSet.newSpecs)
		transactionCards(cardSet.verifier)
		transactionCards(cardSet.warning)
		transactionCards(cardSet.typesInfo)
	}
}

private fun LazyListScope.transactionCards(
	transactions: List<TransactionCard>?,
) {
	transactions?.let {
		items(it.size) { item ->
			TransactionElement(
				card = it[item]
			)
		}
	}
}
