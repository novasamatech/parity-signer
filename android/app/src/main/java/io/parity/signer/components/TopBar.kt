package io.parity.signer.components

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import io.parity.signer.SignerScreen

@Composable
fun TopBar(currentScreen: SignerScreen, navBack: () -> Unit ) {
	Surface(color = MaterialTheme.colors.background) {
		Row(
			horizontalArrangement = Arrangement.SpaceEvenly,
			modifier = Modifier.fillMaxWidth()
		) {
			Text( currentScreen.name )
			Text("Shield")
		}
	}
}
