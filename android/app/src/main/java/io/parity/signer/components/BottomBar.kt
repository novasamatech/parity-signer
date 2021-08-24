package io.parity.signer.components

import android.widget.Toast
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material.Button
import androidx.compose.material.ButtonDefaults
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import io.parity.signer.models.SignerDataModel

@Composable
fun BottomBar(
	signerDataModel: SignerDataModel,
	navToKeys: () -> Unit,
	navToSettings: () -> Unit
) {
	Row(
		horizontalArrangement = Arrangement.SpaceEvenly,
		modifier = Modifier.fillMaxWidth()
	) {
		Button(
			colors = ButtonDefaults.buttonColors(
				backgroundColor = MaterialTheme.colors.background,
				contentColor = MaterialTheme.colors.onBackground,
			),
			onClick = navToKeys
		) {
			Text(text = "Key manager")
		}
		Button(
			colors = ButtonDefaults.buttonColors(
				backgroundColor = MaterialTheme.colors.background,
				contentColor = MaterialTheme.colors.onBackground
			),
			onClick = navToSettings
		) {
			Text(text = "Settings")
		}
	}
}
