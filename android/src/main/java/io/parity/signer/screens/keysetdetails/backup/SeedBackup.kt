package io.parity.signer.screens.keysetdetails.backup

import android.content.res.Configuration
import androidx.compose.foundation.layout.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.BottomSheetHeader
import io.parity.signer.components.base.BottomSheetSubtitle
import io.parity.signer.models.Callback
import io.parity.signer.models.SignerDataModel
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
	var seedPhrase by remember { mutableStateOf("") }
	Column() {
		//header
		BottomSheetHeader(title = model.seedName) {
			//todo dmitry close action
		}
		// phrase
		BottomSheetSubtitle(R.string.subtitle_secret_recovery_phrase)
		BackupPhraseBox(seedPhrase)
		//derived keys
		BottomSheetSubtitle(R.string.subtitle_derived_keys)
		for (item in model.derivations) {

		}
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
