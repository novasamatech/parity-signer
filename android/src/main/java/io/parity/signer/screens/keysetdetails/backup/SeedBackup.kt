package io.parity.signer.screens.keysetdetails.backup

import android.content.res.Configuration
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.size
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.models.Callback
import io.parity.signer.models.KeySetDetailsModel
import io.parity.signer.models.SignerDataModel
import io.parity.signer.screens.keysetdetails.export.KeySetDetailsExportResultBottomSheet
import io.parity.signer.ui.BottomSheetWrapperRoot
import io.parity.signer.ui.theme.SignerNewTheme


@Composable
fun SeedBackupFullOverlayScreen(
	model: SeedBackupModel,
	singleton: SignerDataModel,
) {
	BottomSheetWrapperRoot {
		SeedBackupBottomSheet(model, singleton, {})
	}
}

@Composable
fun SeedBackupBottomSheet(
	model: SeedBackupModel,
	singleton: SignerDataModel,
	onClose: Callback,
) {
	Column() {
		//header
		Row() {

		}
		// phrase


		//derived keys
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
private fun PreviewKeySetDetailsExportResultBottomSheet() {
	SignerNewTheme {
		Box(modifier = Modifier.size(350.dp, 700.dp)) {
//			SeedBackupBottomSheet(model, selected, {}) //todo dmitry
		}
	}
}
