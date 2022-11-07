package io.parity.signer.screens.keysetdetails.backup

import android.content.res.Configuration
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalInspectionMode
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.BottomSheetHeader
import io.parity.signer.components.base.BottomSheetSubtitle
import io.parity.signer.components.items.SlimKeyItem
import io.parity.signer.components.sharedcomponents.CircularCountDownTimer
import io.parity.signer.models.*
import io.parity.signer.screens.keydetails.exportprivatekey.PrivateKeyExportModel
import io.parity.signer.ui.BottomSheetWrapperRoot
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.uniffi.ErrorDisplayed


@Composable
fun SeedBackupFullOverlayScreen(
	model: SeedBackupModel,
	singleton: SignerDataModel,
	onClose: Callback,
) {
	BottomSheetWrapperRoot {
		SeedBackupBottomSheet(model, singleton::getSeedPhraseForBackup, onClose)
	}
	CircularCountDownTimer(
		PrivateKeyExportModel.SHOW_PRIVATE_KEY_TIMEOUT,
		stringResource(R.string.seed_backup_autohide_title),
		onClose,
	)
}

@Composable
fun SeedBackupBottomSheet(
	model: SeedBackupModel,
	getSeedPhraseForBackup: suspend (String) -> String?,
	onClose: Callback,
) {
	var seedPhrase by remember { mutableStateOf("") }
	if (LocalInspectionMode.current) {
		seedPhrase = "sample test data set words for preview"
	}
	Column() {
		//header
		BottomSheetHeader(title = model.seedName, onCloseClicked = onClose)
		Column(
			modifier = Modifier
				.verticalScroll(rememberScrollState()),
		) {
			// phrase
			BottomSheetSubtitle(R.string.subtitle_secret_recovery_phrase)
			BackupPhraseBox(seedPhrase)
			//derived keys
			BottomSheetSubtitle(R.string.subtitle_derived_keys)
			for (item in model.derivations) {
				SlimKeyItem(model = item)
			}
		}
	}

	LaunchedEffect(Unit) {
		val phrase = getSeedPhraseForBackup(model.seedName)
		if (phrase == null) {
			submitErrorState("error authenticate to get backup data")
			onClose()
		} else {
			seedPhrase = phrase
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
	val model = KeySetDetailsModel.createStub().toSeedBackupModel()
	SignerNewTheme {
		Box(modifier = Modifier.size(350.dp, 700.dp)) {
			SeedBackupBottomSheet(model, { _ -> " some long workds" }, {})
		}
	}
}
