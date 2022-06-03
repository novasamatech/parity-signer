package io.parity.signer.components.transactionCards

import androidx.compose.foundation.layout.Row
import androidx.compose.runtime.Composable
import io.parity.signer.components.Identicon
import io.parity.signer.uniffi.MTypesInfo

@Composable
fun TCTypesInfo(types: MTypesInfo) {
	Row {
		types.typesIdPic?.let { Identicon (identicon = it) }
		TCNameValueTemplate(name = "Types hash:", value = types.typesHash ?: "")
	}
}
