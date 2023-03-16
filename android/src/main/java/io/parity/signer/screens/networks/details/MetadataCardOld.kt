package io.parity.signer.screens.networks.details

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.width
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.components.IdentIcon
import io.parity.signer.components.toImageContent
import io.parity.signer.domain.BASE58_STYLE_ABBREVIATE
import io.parity.signer.domain.abbreviateString
import io.parity.signer.uniffi.MMetadataRecord

@Composable
fun MetadataCardOld(metadataRecord: MMetadataRecord) {
	Row {
		IdentIcon(identicon = metadataRecord.metaIdPic.toImageContent())
		Column {
			Text("version")
			Text(metadataRecord.specsVersion)
		}
		Spacer(Modifier.width(20.dp))
		Column {
			Text("hash")
			Text(metadataRecord.metaHash.abbreviateString(BASE58_STYLE_ABBREVIATE))
		}
	}
}
