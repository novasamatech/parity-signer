package io.parity.signer.screens

import android.widget.Toast
import android.widget.Toast.LENGTH_LONG
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.fragment.app.FragmentActivity
import io.parity.signer.components.NetworkSelector
import io.parity.signer.components.SeedSelector
import io.parity.signer.modals.NewSeedModal
import io.parity.signer.modals.SeedBackup
import io.parity.signer.models.SignerDataModel

/**
 * Key manager screen; here all key/identity/seed creation and deletion
 * operations should happen. This is final point in navigation:
 * all subsequent interactions should be in modals or drop-down menus
 */
@Composable
fun KeyManager(signerDataModel: SignerDataModel) {
	val context = LocalContext.current as FragmentActivity
	val seedNames = signerDataModel.seedNames.observeAsState()
	val newSeedScreen = signerDataModel.newSeedScreen.observeAsState()
	val backupSeedPhrase = signerDataModel.backupSeedPhrase.observeAsState()

	if (newSeedScreen.value as Boolean) {
		NewSeedModal(signerDataModel)
	} else if (backupSeedPhrase.value?.isEmpty() as Boolean) {
		Column {
			NetworkSelector(signerDataModel = signerDataModel)
			Row (
				horizontalArrangement = Arrangement.SpaceBetween,
				modifier = Modifier.fillMaxSize()
				) {
				SeedSelector(signerDataModel = signerDataModel)
				Button(
					colors = ButtonDefaults.buttonColors(
						backgroundColor = MaterialTheme.colors.background,
						contentColor = MaterialTheme.colors.onBackground
					),
					//Copypaste for toast
					onClick = {
						signerDataModel.newSeedScreenEngage()
					}
				) {
					Text(text = "New seed")
				}
			}
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
		}
	} else {
		SeedBackup(signerDataModel = signerDataModel)
	}

}
