package io.parity.signer.components

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.unit.dp
import io.parity.signer.models.SignerDataModel
import io.parity.signer.ui.theme.Crypto400
import org.json.JSONArray
import org.json.JSONObject

@Composable
fun TransactionPreviewField(transaction: JSONArray) {

	LazyColumn(
		modifier = Modifier
			.aspectRatio(1f)
			.padding(8.dp)
			.border(
				BorderStroke(1.dp, Crypto400),
				RoundedCornerShape(8.dp)
			)
			.clip(RoundedCornerShape(8.dp))
			.padding(8.dp)
	) {
		items(transaction.length()) { item ->
			TransactionCard(
				card = transaction.getJSONObject(item)
			)
		}
	}
}
