package io.parity.signer.components.transactionCards

import androidx.compose.runtime.Composable
import org.json.JSONObject

@Composable
fun TCBitVec(bitVec: String) {
	TCNameValueTemplate(name = bitVec, value = bitVec)
}
