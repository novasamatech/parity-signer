package io.parity.signer.components

import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import org.json.JSONObject

@Composable
fun TransactionCard(card: JSONObject) {
	Text(card.toString())
}
