package io.parity.signer.components

import android.util.Log
import android.widget.ImageView
import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.AccountCircle
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ImageBitmap
import io.parity.signer.models.SignerDataModel
import org.json.JSONObject

/**
 * A card to show key info; only visual things.
 * TODO: paint root keys in scary colors
 */
@Composable
fun KeyCard(identity: JSONObject, signerDataModel: SignerDataModel) {
	Row {
		Image(signerDataModel.getIdenticon(identity.get("ss58").toString(), 16), "identicon")
		Column {
			Text(identity.get("name").toString())
			Row {
				Text(identity.get("seed_name").toString())
				Text(identity.get("path").toString())
			}
			Text(identity.get("ss58").toString())
			Text(identity.get("public_key").toString())
		}
	}
}
