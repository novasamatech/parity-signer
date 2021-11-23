package io.parity.signer.components

import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.ButtonID
import io.parity.signer.SignerScreen
import io.parity.signer.models.SignerDataModel
import io.parity.signer.ui.theme.Bg000

/**
 * Bar to be shown on the bottom of screen;
 */
@Composable
fun BottomBar(
	signerDataModel: SignerDataModel,
) {
	BottomAppBar (
		backgroundColor = Bg000,
		elevation = 0.dp,
		modifier = Modifier.height(54.dp)) {
		BottomBarButton(
			signerDataModel = signerDataModel,
			image = Icons.Default.Home,
			buttonID = ButtonID.NavbarLog
		)
		BottomBarButton(
			signerDataModel = signerDataModel,
			image = Icons.Default.Star,
			buttonID = ButtonID.NavbarScan
		)
		BottomBarButton(
			signerDataModel = signerDataModel,
			image = Icons.Default.AccountCircle,
			buttonID = ButtonID.NavbarKeys
		)
		BottomBarButton(
			signerDataModel = signerDataModel,
			image = Icons.Default.Settings,
			buttonID = ButtonID.NavbarSettings
		)
	}
}
