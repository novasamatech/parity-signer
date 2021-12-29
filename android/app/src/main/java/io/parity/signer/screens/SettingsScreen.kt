package io.parity.signer.screens

import android.content.Context
import android.os.Build
import androidx.compose.foundation.Image
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.scale
import androidx.compose.ui.unit.dp
import io.parity.signer.models.SignerDataModel

/**
 * Settings screen; General purpose stuff like legal info, networks management
 * and history should be here. This is final point in navigation:
 * all subsequent interactions should be in modals or drop-down menus
 */
@Composable
fun SettingsScreen(signerDataModel: SignerDataModel) {

	Column {
		Row(
			Modifier.clickable {
				signerDataModel.wipe()
				signerDataModel.totalRefresh()
			}
		) { Text("Wipe Signer") }
		Spacer(modifier = Modifier.padding(10.dp))
		Row(Modifier.clickable { signerDataModel.jailbreak() }
		) { Text("Wipe general certificate") }
		Spacer(modifier = Modifier.padding(10.dp))
		Text("General certificate")
		/*
		Row {
			Image(
				signerDataModel.getHexIdenticon(
					generalCertificate.value?.optString(
						"hex"
					) ?: "", 64
				), "identicon", modifier = Modifier.scale(0.75f)
			)
			Column {
				Text(generalCertificate.value?.optString("encryption") ?: "none")
				Text(generalCertificate.value?.optString("hex") ?: "")
			}
		}*/
		Spacer(modifier = Modifier.padding(10.dp))
		Text(
			"Hardware seed protection: " + signerDataModel.isStrongBoxProtected()
				.toString()
		)
		Text("Version: " + signerDataModel.getAppVersion())
	}
}
