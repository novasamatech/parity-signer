package io.parity.signer.components

import androidx.compose.foundation.layout.Column
import androidx.compose.material.Icon
import androidx.compose.material.IconButton
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Lock
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.graphics.vector.ImageVector
import io.parity.signer.SignerScreen
import io.parity.signer.models.SignerDataModel

/**
 * Unified bottom bar button view
 */
@Composable
fun BottomBarButton(signerDataModel: SignerDataModel, image: ImageVector, screen: SignerScreen) {
	IconButton(onClick = {
		signerDataModel.totalRefresh()
		signerDataModel.navigate(screen)
	}) {
		Column(
			horizontalAlignment = Alignment.CenterHorizontally
		) {
			Icon(image, contentDescription = screen.name)
			Text(screen.name)
		}
	}
}

