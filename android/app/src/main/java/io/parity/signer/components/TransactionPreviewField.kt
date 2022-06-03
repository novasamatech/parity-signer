package io.parity.signer.components

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

fun transactionCards(
	scope: LazyListScope,
	transactions: List<TransactionCard>?,
) {
	transactions?.let {
		scope.items(it.size) { item ->
			TransactionCard(
				card = it[item]
			)
		}
	}
}

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
		transactionCards(this, cardSet.author)
		transactionCards(this, cardSet.error)
		transactionCards(
			this,
			cardSet.extensions
		)
		transactionCards(
			this,
			cardSet.importingDerivations
		)
		transactionCards(this, cardSet.message)
		transactionCards(this, cardSet.meta)
		transactionCards(this, cardSet.method)
		transactionCards(this, cardSet.newSpecs)
		transactionCards(this, cardSet.verifier)
		transactionCards(this, cardSet.warning)
		transactionCards(
			this,
			cardSet.typesInfo
		)
	}
}
