package io.parity.signer.modals

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Column
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.res.painterResource
import io.parity.signer.R

@Composable
fun WaitingScreen() {
	Column {
		Text("Please wait...")
		Image(
			painter = painterResource(id = R.drawable.icon),
			contentDescription = "Icon"
		)
	}
}
