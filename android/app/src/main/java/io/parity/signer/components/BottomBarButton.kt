package io.parity.signer.components

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.width
import androidx.compose.material.Icon
import androidx.compose.material.IconButton
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Lock
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.unit.dp
import io.parity.signer.ButtonID
import io.parity.signer.SignerScreen
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.pushButton
import io.parity.signer.ui.theme.Text300
import io.parity.signer.ui.theme.Text400
import io.parity.signer.ui.theme.Text600

/**
 * Unified bottom bar button view
 */
@Composable
fun BottomBarButton(
	signerDataModel: SignerDataModel,
	image: ImageVector,
	buttonID: ButtonID
) {
	val selected = signerDataModel.footerButton.observeAsState().value == buttonID.getName()
	val tint = if (selected) {
		Text600
	} else {
		Text300
	}
	val color = if (selected) {
		Text600
	} else {
		Text400
	}
	IconButton(
		onClick = {
			signerDataModel.totalRefresh()
			signerDataModel.pushButton(buttonID)
			//signerDataModel.navigate(buttonID.getName())
		},
		modifier = Modifier.width(66.dp)
	) {
		Column(
			horizontalAlignment = Alignment.CenterHorizontally,
			modifier = Modifier.width(66.dp)
		) {
			Icon(image, contentDescription = buttonID.getName(), tint = tint)
			Text(
				buttonID.getName(),
				color = color,
				style = MaterialTheme.typography.subtitle2
			)
		}
	}
}

