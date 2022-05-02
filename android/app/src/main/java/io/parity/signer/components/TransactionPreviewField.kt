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
import io.parity.signer.uniffi.TransactionAuthor
import io.parity.signer.uniffi.TransactionCard
import io.parity.signer.uniffi.TransactionCardSet

fun TransactionCards(
	scope: LazyListScope,
	transactions: List<TransactionCard>?,
	authorInfo: TransactionAuthor?
) {
	transactions?.let {
		scope.items(it.size) { item ->
			TransactionCard(
				card = it[item], authorInfo = authorInfo
			)
		}
	}
}

@Composable
fun TransactionPreviewField(
	cardSet: TransactionCardSet,
	authorInfo: TransactionAuthor?
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
		TransactionCards(this, cardSet.author, authorInfo)
		TransactionCards(this, cardSet.error, authorInfo)
		TransactionCards(
			this,
			cardSet.extensions,
			authorInfo
		)
		TransactionCards(
			this,
			cardSet.importingDerivations,
			authorInfo
		)
		TransactionCards(this, cardSet.message, authorInfo)
		TransactionCards(this, cardSet.meta, authorInfo)
		TransactionCards(this, cardSet.method, authorInfo)
		TransactionCards(this, cardSet.newSpecs, authorInfo)
		TransactionCards(this, cardSet.verifier, authorInfo)
		TransactionCards(this, cardSet.warning, authorInfo)
		TransactionCards(
			this,
			cardSet.typesInfo,
			authorInfo
		)
	}
}
