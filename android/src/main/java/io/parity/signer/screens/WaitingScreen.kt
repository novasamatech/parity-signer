package io.parity.signer.screens

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Column
import androidx.compose.material.CircularProgressIndicator
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.res.painterResource
import io.parity.signer.R

/**
 * Screen that might be shown when user should wait for something
 */
@Composable
fun WaitingScreen() {
	Column {
		Text("Please wait...")
		CircularProgressIndicator()
		Image(
			painter = painterResource(id = R.drawable.icon),
			contentDescription = "Icon"
		)
	}
}
