package io.parity.signer.components

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import io.parity.signer.SignerScreen

/**
 * Top bar - navigation bar
 * Should get navigation callbacks, current navigation state, network safety
 * indicator and history alert
 */
@Composable
fun TopBar(currentScreen: SignerScreen, navBack: () -> Unit ) {
	Surface(color = MaterialTheme.colors.background) {
		Row(
			horizontalArrangement = Arrangement.SpaceEvenly,
			modifier = Modifier.fillMaxWidth()
		) {
			Button(
				colors = ButtonDefaults.buttonColors(
					backgroundColor = MaterialTheme.colors.background,
					contentColor = MaterialTheme.colors.onBackground
				),
				onClick = navBack
			) {
				Text(currentScreen.name)
			}
			Text("Shield")
		}
	}
}
