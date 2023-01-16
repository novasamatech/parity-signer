package io.parity.signer.screens.scan.camera.bananasplit

import android.content.res.Configuration
import androidx.compose.foundation.layout.Column
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.tooling.preview.Preview
import io.parity.signer.components.base.ScreenHeaderWithButton
import io.parity.signer.models.Callback
import io.parity.signer.ui.theme.SignerNewTheme


@Composable
fun BananaSplitPasswordBottomSheet(
	onClose: Callback,
	onDone: Callback,
) {
	val canProceed = remember { mutableStateOf(false) }

	Column() {
		ScreenHeaderWithButton(
			canProceed = canProceed.value,
			title = "",
			onClose = onClose,
			onDone = {}, //todo banana
		)


	}
}


@Preview(
	name = "light", group = "general", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "general",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewDerivationKeysHelpBottomSheet() {
	SignerNewTheme {
		BananaSplitPasswordBottomSheet(onClose = {},
			onDone = {},)
	}
}
