package io.parity.signer.screens.scan.transaction.transactionElements

import androidx.compose.foundation.layout.Row
import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import io.parity.signer.R
import io.parity.signer.components.networkicon.IdentIconImage
import io.parity.signer.uniffi.MscId

@Composable
fun TCAuthorPlain(author: MscId) {
	Row {
		IdentIconImage(author.identicon)
		TCNameValueElement(
			name = stringResource(R.string.transaction_field_from),
			value = author.base58,
		)
	}
}
