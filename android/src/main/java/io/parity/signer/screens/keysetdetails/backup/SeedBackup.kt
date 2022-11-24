package io.parity.signer.screens.keysetdetails.backup

import android.content.res.Configuration
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalInspectionMode
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.BottomSheetHeader
import io.parity.signer.components.base.BottomSheetSubtitle
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.components.items.SlimKeyItem
import io.parity.signer.components.sharedcomponents.SnackBarCircularCountDownTimer
import io.parity.signer.models.BASE58_STYLE_ABBREVIATE
import io.parity.signer.models.Callback
import io.parity.signer.models.KeySetDetailsModel
import io.parity.signer.models.abbreviateString
import io.parity.signer.screens.keydetails.exportprivatekey.PrivateKeyExportModel
import io.parity.signer.ui.BottomSheetWrapperRoot
import io.parity.signer.ui.theme.SignerNewTheme


@Composable
fun SeedBackupFullOverlayBottomSheet(
	model: SeedBackupModel,
	getSeedPhraseForBackup: suspend (String) -> String?,
	onClose: Callback,
) {
	BottomSheetWrapperRoot(onClosedAction = onClose) {
		SeedBackupBottomSheet(model, getSeedPhraseForBackup, onClose)
	}
	Row(modifier = Modifier.fillMaxSize()) {
		SnackBarCircularCountDownTimer(
			PrivateKeyExportModel.SHOW_PRIVATE_KEY_TIMEOUT,
			stringResource(R.string.seed_backup_autohide_title),
			Modifier.align(Alignment.Bottom),
			onClose,
		)
	}
}

@Composable
private fun SeedBackupBottomSheet(
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
		BottomSheetHeader(
			title = model.seedName,
			subtitile = model.seedBase58.abbreviateString(BASE58_STYLE_ABBREVIATE),
			onCloseClicked = onClose,
		)
		SignerDivider(padding = 24.dp)
		Column(
			modifier = Modifier
				.verticalScroll(rememberScrollState()),
		) {
			// phrase
			Spacer(modifier = Modifier.padding(top = 14.dp))
			BottomSheetSubtitle(R.string.subtitle_secret_recovery_phrase)
			Spacer(modifier = Modifier.padding(top = 14.dp))
			BackupPhraseBox(seedPhrase)
			//derived keys
			Spacer(modifier = Modifier.padding(top = 22.dp))
			BottomSheetSubtitle(R.string.subtitle_derived_keys)
			Spacer(modifier = Modifier.padding(top = 14.dp))
			for (index in 0..model.derivations.lastIndex) {
				SlimKeyItem(model = model.derivations[index])
				if (index != model.derivations.lastIndex) {
					SignerDivider(padding = 24.dp)
				}
			}
			Spacer(modifier = Modifier.size(height = 80.dp, width = 1.dp))
		}
	}

	LaunchedEffect(Unit) {
		val phrase = getSeedPhraseForBackup(model.seedName)
		if (phrase != null) {
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
private fun PreviewSeedBackupBottomSheet() {
	val model = KeySetDetailsModel.createStub().toSeedBackupModel()
	SignerNewTheme {
		SeedBackupBottomSheet(model,
			{ _ -> " some long words some some" }, {})
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
private fun PreviewSeedBackupFullOverlayScreen() {
	val model = KeySetDetailsModel.createStub().toSeedBackupModel()
	SignerNewTheme {
		Box(modifier = Modifier.size(350.dp, 700.dp)) {
			SeedBackupFullOverlayBottomSheet(model,
				{ _ -> " some long words some some" }, {})
		}
	}
}
