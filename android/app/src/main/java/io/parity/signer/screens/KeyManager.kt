package io.parity.signer.screens

import android.widget.ImageButton
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.AddCircle
import androidx.compose.runtime.*
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.scale
import androidx.compose.ui.platform.LocalContext
import androidx.fragment.app.FragmentActivity
import io.parity.signer.KeyManagerModal
import io.parity.signer.components.KeySelector
import io.parity.signer.components.NetworkSelector
import io.parity.signer.components.SeedCard
import io.parity.signer.components.SeedSelector
import io.parity.signer.modals.*
import io.parity.signer.models.SignerDataModel

/**
 * Key manager screen; here all key/identity/seed creation and deletion
 * operations should happen. This is final point in navigation:
 * all subsequent interactions should be in modals or drop-down menus
 */
@Composable
fun KeyManager(signerDataModel: SignerDataModel) {
	val keyManagerModal = signerDataModel.keyManagerModal.observeAsState()
	val seedName = signerDataModel.selectedSeed.observeAsState()

	when (keyManagerModal.value) {
		KeyManagerModal.None -> {
			Column {
				Row(Modifier.clickable {
					signerDataModel.selectKey(signerDataModel.getRootIdentity(seedName.value?:""))
					signerDataModel.exportPublicKeyEngage()
				}) {
				SeedCard(
					seedName = seedName.value ?: "",
					seedSelector = true,
					signerDataModel = signerDataModel
				)}
				NetworkSelector(signerDataModel = signerDataModel)
				Row(modifier = Modifier.fillMaxWidth(1f)) {
					Text("DERIVED KEYS")
					Spacer(Modifier.weight(1f, true))
					IconButton(onClick = { signerDataModel.newKeyScreenEngage() }) {
						Icon(Icons.Default.AddCircle, contentDescription = "New derived key")
					}
				}
				KeySelector(signerDataModel)
			}
		}
		KeyManagerModal.SeedBackup -> {
			SeedBackup(signerDataModel)
		}
		KeyManagerModal.NewSeed -> {
			NewSeedModal(signerDataModel)
		}
		KeyManagerModal.NewKey -> {
			NewKeyModal(signerDataModel, false)
		}
		KeyManagerModal.ShowKey -> {
			ExportPublicKey(signerDataModel)
		}
		KeyManagerModal.KeyDeleteConfirm -> {
			KeyDelete(signerDataModel)
		}
		null -> WaitingScreen()
		KeyManagerModal.SeedSelector -> SeedManager(signerDataModel = signerDataModel)
		KeyManagerModal.NetworkManager -> NetworkManager(signerDataModel = signerDataModel)
		KeyManagerModal.NetworkDetails -> NetworkDetails(signerDataModel = signerDataModel)
	}
}

/*
Button(
				colors = ButtonDefaults.buttonColors(
					backgroundColor = MaterialTheme.colors.background,
					contentColor = MaterialTheme.colors.onBackground
				),
				//Copypaste for toast
				onClick = {
					Toast.makeText(
						signerDataModel.context,
						signerDataModel.callNative("000000"),
						LENGTH_LONG
					).show()
				}
			) {
				Text(text = "Settings")
			}
			Button(
				colors = ButtonDefaults.buttonColors(
					backgroundColor = MaterialTheme.colors.background,
					contentColor = MaterialTheme.colors.onBackground
				),
				onClick = { signerDataModel.authentication.authenticate(context) {} }
			) {
				Text(text = "Eat me!")
			}

*/
