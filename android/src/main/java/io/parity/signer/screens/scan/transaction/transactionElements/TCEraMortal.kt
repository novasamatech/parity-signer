package io.parity.signer.screens.scan.transaction.transactionElements

import androidx.compose.foundation.layout.Column
import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import io.parity.signer.R
import io.parity.signer.uniffi.MscEraMortal

@Composable
fun TCEraMortal(era: MscEraMortal) {
	Column {
		TCNameValueElement(name = stringResource(R.string.transaction_field_phrase), value = era.phase)
		TCNameValueElement(name = stringResource(R.string.transaction_field_period), value = era.period)
	}
}
