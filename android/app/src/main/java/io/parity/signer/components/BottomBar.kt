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
		modifier = Modifier.height(57.dp)) {
		Spacer(Modifier.weight(1f))
		BottomBarButton(
			signerDataModel = signerDataModel,
			image = Icons.Default.CalendarViewDay,
			buttonID = ButtonID.NavbarLog
		)
		Spacer(Modifier.weight(1f))
		BottomBarButton(
			signerDataModel = signerDataModel,
			image = Icons.Default.CropFree,
			buttonID = ButtonID.NavbarScan
		)
		Spacer(Modifier.weight(1f))
		BottomBarButton(
			signerDataModel = signerDataModel,
			image = Icons.Default.Pattern,
			buttonID = ButtonID.NavbarKeys
		)
		Spacer(Modifier.weight(1f))
		BottomBarButton(
			signerDataModel = signerDataModel,
			image = Icons.Default.Settings,
			buttonID = ButtonID.NavbarSettings
		)
		Spacer(Modifier.weight(1f))
	}
}
