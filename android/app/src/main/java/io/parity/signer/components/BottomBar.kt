package io.parity.signer.components

import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.SignerScreen
import io.parity.signer.models.SignerDataModel

/**
 * Bar to be shown on the bottom of screen;
 */
@Composable
fun BottomBar(
	signerDataModel: SignerDataModel,
) {
	BottomAppBar (elevation = 0.dp,
		modifier = Modifier.height(80.dp)) {
		BottomBarButton(
			signerDataModel = signerDataModel,
			image = Icons.Default.Home,
			screen = SignerScreen.Log
		)
		Spacer(Modifier.weight(1f, true))
		BottomBarButton(
			signerDataModel = signerDataModel,
			image = Icons.Default.Star,
			screen = SignerScreen.Scan
		)
		Spacer(Modifier.weight(1f, true))
		BottomBarButton(
			signerDataModel = signerDataModel,
			image = Icons.Default.AccountCircle,
			screen = SignerScreen.Keys
		)
		Spacer(Modifier.weight(1f, true))
		BottomBarButton(
			signerDataModel = signerDataModel,
			image = Icons.Default.Settings,
			screen = SignerScreen.Settings
		)
	}
}
