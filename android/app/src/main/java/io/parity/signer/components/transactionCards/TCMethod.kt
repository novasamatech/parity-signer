package io.parity.signer.components.transactionCards

import androidx.compose.foundation.layout.Row
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import io.parity.signer.uniffi.MscCall
import org.json.JSONObject

@Composable
fun TCMethod(payload: MscCall) {
	Row {
		Text("Method: ")
		Text(payload.methodName)
	}
}
