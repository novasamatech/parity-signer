package io.parity.signer.screens.scan.transaction.transactionElements

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import io.parity.signer.components.IdentIcon
import io.parity.signer.models.toBytes
import io.parity.signer.ui.theme.Crypto400
import io.parity.signer.ui.theme.CryptoTypography
import io.parity.signer.ui.theme.Text400
import io.parity.signer.ui.theme.Text600
import io.parity.signer.uniffi.MMetadataRecord

@Composable
fun TCMeta(meta: MMetadataRecord) {
	//todo dmitry redesign and below
	Row {
		IdentIcon(identicon = meta.metaIdPic)
		Column {
			Text(
				"Add metadata",
				style = MaterialTheme.typography.body2,
				color = MaterialTheme.colors.Text600
			)
			Text(meta.specsVersion,
				style = CryptoTypography.body2,
				color = MaterialTheme.colors.Crypto400
			)
			Text(
				meta.metaHash,
				style = CryptoTypography.body2,
				color = MaterialTheme.colors.Text400
			)
		}
	}
}
