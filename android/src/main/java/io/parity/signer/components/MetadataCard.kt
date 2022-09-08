package io.parity.signer.components

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.width
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.models.abbreviateString
import io.parity.signer.uniffi.MMetadataRecord

@Composable
fun MetadataCard(metadataRecord: MMetadataRecord) {
	Row {
		Identicon(identicon = metadataRecord.metaIdPic)
		Column {
			Text("version")
			Text(metadataRecord.specsVersion)
		}
		Spacer(Modifier.width(20.dp))
		Column {
			Text("hash")
			Text(metadataRecord.metaHash.abbreviateString(8))
		}
	}
}
