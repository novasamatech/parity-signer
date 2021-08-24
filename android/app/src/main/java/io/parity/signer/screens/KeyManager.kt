package io.parity.signer.screens

import android.widget.Toast
import android.widget.Toast.LENGTH_LONG
import androidx.compose.foundation.layout.Column
import androidx.compose.material.Button
import androidx.compose.material.ButtonDefaults
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import io.parity.signer.models.SignerDataModel

/**
 * Key manager screen; here all key/identity/seed creation and deletion
 * operations should happen. This is final point in navigation:
 * all subsequent interactions should be in modals or drop-down menus
 */
@Composable
fun KeyManager(signerDataModel: SignerDataModel) {
	Column {
		Button(
			colors = ButtonDefaults.buttonColors(
				backgroundColor = MaterialTheme.colors.background,
				contentColor = MaterialTheme.colors.onBackground
			),
			onClick = { Toast.makeText(signerDataModel.context, signerDataModel.callNative("000000"), LENGTH_LONG).show() }
		) {
			Text(text = "Settings")
		}
	}

}
