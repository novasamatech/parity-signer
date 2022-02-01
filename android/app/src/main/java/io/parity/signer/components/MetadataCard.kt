package io.parity.signer.components

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
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
		Column {
			Text("hash")
			Text(meta.optString("meta.hash").abbreviateString(8))
		}
	}
}
