package io.parity.signer.screens.scan.transaction.transactionElements

import androidx.compose.foundation.layout.Column
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import io.parity.signer.R
import io.parity.signer.models.encodeHex
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.textSecondary
import io.parity.signer.uniffi.MscTxSpecPlain

@Composable
fun TCUnknownNetwork(specsPlain: MscTxSpecPlain) {
	Column {
		Text(
			text = stringResource(R.string.transaction_unknown_network_header),
			style = SignerTypeface.BodyL,
			color = MaterialTheme.colors.textSecondary,
		)
		TCNameValueElement(
			name = stringResource(R.string.transaction_unknown_network_genesis_hash),
			value = specsPlain.networkGenesisHash.toUByteArray()
				.toByteArray().encodeHex()
		)
		TCNameValueElement(
			name = stringResource(R.string.transaction_unknown_network_version),
			value = specsPlain.version
		)
		TCNameValueElement(
			name = stringResource(R.string.transaction_unknown_network_tx_version),
			value = specsPlain.txVersion
		)
	}
}
