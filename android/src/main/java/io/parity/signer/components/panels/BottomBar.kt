package io.parity.signer.components.panels

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.material.BottomAppBar
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.CalendarViewDay
import androidx.compose.material.icons.filled.CropFree
import androidx.compose.material.icons.filled.Pattern
import androidx.compose.material.icons.filled.Settings
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.unit.dp
import io.parity.signer.domain.SignerDataModel
import io.parity.signer.domain.navigate
import io.parity.signer.ui.theme.Bg000
import io.parity.signer.ui.theme.Text300
import io.parity.signer.ui.theme.Text400
import io.parity.signer.ui.theme.Text600
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.actionGetName

/**
 * Bar to be shown on the bottom of screen;
 */
@Composable
@Deprecated("not used")
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
				image = Icons.Default.Pattern,
				action = Action.NAVBAR_KEYS
			)
			BottomBarButton(
				signerDataModel = signerDataModel,
				image = Icons.Default.CropFree,
				action = Action.NAVBAR_SCAN
			)
			BottomBarButton(
				signerDataModel = signerDataModel,
				image = Icons.Default.CalendarViewDay,
				action = Action.NAVBAR_LOG
			)
			BottomBarButton(
				signerDataModel = signerDataModel,
				image = Icons.Default.Settings,
				action = Action.NAVBAR_SETTINGS
			)
		}
	}
}



/**
 * Unified bottom bar button view
 */
@Composable
fun BottomBarButton(
	signerDataModel: SignerDataModel,
	image: ImageVector,
	action: Action,
) {
	val selected =
		signerDataModel.actionResult.collectAsState().value.footerButton == actionGetName(action)
	val tint = if (selected) {
		MaterialTheme.colors.Text600
	} else {
		MaterialTheme.colors.Text300
	}
	val color = if (selected) {
		MaterialTheme.colors.Text600
	} else {
		MaterialTheme.colors.Text400
	}
	Column(
		horizontalAlignment = Alignment.CenterHorizontally,
		modifier = Modifier
			.clickable(onClick = {
				signerDataModel.navigate(action)
			})
			.width(66.dp)
	) {
		Icon(image, contentDescription = actionGetName(action).toString(), tint = tint)
		Text(
			actionGetName(action).toString(),
			color = color,
			style = MaterialTheme.typography.subtitle2
		)
	}
}


