package io.parity.signer.components

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.material.BottomAppBar
import androidx.compose.material.MaterialTheme
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.CalendarViewDay
import androidx.compose.material.icons.filled.CropFree
import androidx.compose.material.icons.filled.Pattern
import androidx.compose.material.icons.filled.Settings
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.parity.signer.models.SignerDataModel
import io.parity.signer.ui.theme.Bg000
import io.parity.signer.uniffi.Action

/**
 * Bar to be shown on the bottom of screen;
 */
@Composable
fun BottomBar(
	signerDataModel: SignerDataModel,
) {
	BottomAppBar(
		backgroundColor = MaterialTheme.colors.Bg000,
		elevation = 0.dp,
		modifier = Modifier.height(56.dp)
	) {
		Row(
			horizontalArrangement = Arrangement.SpaceEvenly,
			modifier = Modifier.fillMaxWidth(1f)
		) {
			BottomBarButton(
				signerDataModel = signerDataModel,
				image = Icons.Default.CalendarViewDay,
				action = Action.NAVBAR_LOG
			)
			BottomBarButton(
				signerDataModel = signerDataModel,
				image = Icons.Default.CropFree,
				action = Action.NAVBAR_SCAN
			)
			BottomBarButton(
				signerDataModel = signerDataModel,
				image = Icons.Default.Pattern,
				action = Action.NAVBAR_KEYS
			)
			BottomBarButton(
				signerDataModel = signerDataModel,
				image = Icons.Default.Settings,
				action = Action.NAVBAR_SETTINGS
			)
		}
	}
}
