package io.parity.signer.screens

import android.widget.Toast
import android.widget.Toast.LENGTH_LONG
import androidx.compose.foundation.layout.Column
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.platform.LocalContext
import androidx.fragment.app.FragmentActivity
import io.parity.signer.components.NetworkSelector
import io.parity.signer.models.SignerDataModel

/**
 * Key manager screen; here all key/identity/seed creation and deletion
 * operations should happen. This is final point in navigation:
 * all subsequent interactions should be in modals or drop-down menus
 */
@Composable
fun KeyManager(signerDataModel: SignerDataModel) {
	val context = LocalContext.current as FragmentActivity

	Column {
		NetworkSelector(signerDataModel = signerDataModel)
		Button(
			colors = ButtonDefaults.buttonColors(
				backgroundColor = MaterialTheme.colors.background,
				contentColor = MaterialTheme.colors.onBackground
			),
			//Copypaste for toast
			onClick = { Toast.makeText(signerDataModel.context, signerDataModel.callNative("000000"), LENGTH_LONG).show() }
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

}
