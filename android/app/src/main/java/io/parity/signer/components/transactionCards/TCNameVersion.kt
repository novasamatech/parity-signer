package io.parity.signer.components.transactionCards

import androidx.compose.runtime.Composable
import io.parity.signer.uniffi.MscNameVersion
import org.json.JSONObject

@Composable
fun TCNameVersion(nameVersion: MscNameVersion) {
	TCNameValueTemplate(name = nameVersion.name, value = nameVersion.name)
}
