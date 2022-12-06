package io.parity.signer.screens.scan.transaction.transactionCards

import androidx.compose.runtime.Composable
import io.parity.signer.uniffi.MscNameVersion

@Composable
fun TCNameVersion(nameVersion: MscNameVersion) {
	TCNameValueTemplate(name = nameVersion.name, value = nameVersion.name)
}
