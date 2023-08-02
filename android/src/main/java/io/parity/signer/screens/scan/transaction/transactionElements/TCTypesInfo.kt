package io.parity.signer.screens.scan.transaction.transactionElements

import androidx.compose.foundation.layout.Row
import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import io.parity.signer.R
import io.parity.signer.components.IdentIcon
import io.parity.signer.components.toImageContent
import io.parity.signer.uniffi.MTypesInfo

@Composable
fun TCTypesInfo(types: MTypesInfo) {
	Row {
		types.typesIdPic?.let {
			IdentIcon(identIcon = it.toImageContent())
		}
		TCNameValueElement(
			name = stringResource(R.string.transaction_types_info_hash),
			value = types.typesHash ?: ""
		)
	}
}
