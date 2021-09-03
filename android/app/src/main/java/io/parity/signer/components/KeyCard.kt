package io.parity.signer.components

import android.util.Log
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.AccountCircle
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import io.parity.signer.models.SignerDataModel
import org.json.JSONObject

@Composable
fun KeyCard(identity: JSONObject, signerDataModel: SignerDataModel) {
	Row {
		Icon(Icons.Default.AccountCircle, contentDescription = "Identicon")
		Column {
			Row {
				Text(identity.get("seed_name").toString())
				Text(identity.get("path").toString())
			}
			Text(identity.get("ss58").toString())
			Text(identity.get("public_key").toString())
		}
	}
}
