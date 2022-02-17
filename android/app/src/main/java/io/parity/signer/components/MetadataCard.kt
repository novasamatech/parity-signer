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
import org.json.JSONObject

@Composable
fun MetadataCard(meta: JSONObject) {
  Row {
  	Identicon(identicon = meta.optString("meta_id_pic"))
		Column {
			Text("version")
			Text(meta.optString("spec_version"))
		}
		Spacer(Modifier.width(20.dp))
		Column {
			Text("hash")
			Text(meta.optString("meta_hash").abbreviateString(8))
		}
	}
}
