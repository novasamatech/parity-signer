package io.parity.signer.screens.keysetdetails.backup

import android.annotation.SuppressLint
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
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.BottomSheetHeader
import io.parity.signer.components.base.BottomSheetSubtitle
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.components.sharedcomponents.SnackBarCircularCountDownTimer
import io.parity.signer.domain.BASE58_STYLE_ABBREVIATE
import io.parity.signer.domain.Callback
import io.parity.signer.domain.KeySetDetailsModel
import io.parity.signer.domain.abbreviateString
import io.parity.signer.screens.keydetails.exportprivatekey.PrivateKeyExportModel
import io.parity.signer.screens.keysetdetails.items.SlimKeyItem
import io.parity.signer.ui.BottomSheetWrapperRoot
import io.parity.signer.ui.theme.SignerNewTheme


@Composable
fun KeySetBackupFullOverlayBottomSheet(
	model: SeedBackupModel,
	getSeedPhraseForBackup: suspend (String) -> String?,
	onClose: Callback,
) {
	val timerSize = remember {
		mutableStateOf(80.dp)
	}

	BottomSheetWrapperRoot(onClosedAction = onClose) {
		KeySetBackupBottomSheet(model, timerSize, getSeedPhraseForBackup, onClose)
	}
	Row(modifier = Modifier.fillMaxSize()) {
		SnackBarCircularCountDownTimer(
			PrivateKeyExportModel.SHOW_PRIVATE_KEY_TIMEOUT,
			stringResource(R.string.seed_backup_autohide_title),
			timerSize,
			Modifier.align(Alignment.Bottom),
			onClose,
		)
	}
}

@Composable
private fun KeySetBackupBottomSheet(
	model: SeedBackupModel,
	timerSize: State<Dp>,
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
			onClose = onClose,
		)
		SignerDivider(sidePadding = 24.dp)
		Column(
			modifier = Modifier
				.verticalScroll(rememberScrollState()),
		) {
			// phrase
			BottomSheetSubtitle(
				R.string.subtitle_secret_recovery_phrase,
				Modifier.padding(top = 14.dp, bottom = 8.dp)
			)
			SeedPhraseBox(seedPhrase)
			//derived keys
			BottomSheetSubtitle(
				R.string.subtitle_derived_keys,
				Modifier.padding(top = 8.dp, bottom = 14.dp)
			)
			for (index in 0..model.derivations.lastIndex) {
				SlimKeyItem(model = model.derivations[index])
				if (index != model.derivations.lastIndex) {
					SignerDivider(sidePadding = 24.dp)
				}
			}
			Spacer(modifier = Modifier.size(height = timerSize.value, width = 1.dp))
		}
	}

	LaunchedEffect(model.seedName) {
		val phrase = getSeedPhraseForBackup(model.seedName)
		if (phrase != null) {
			seedPhrase = phrase
		}
	}
}


@SuppressLint("UnrememberedMutableState")
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
private fun PreviewKeySetBackupBottomSheet() {
	val model = KeySetDetailsModel.createStub().toSeedBackupModel()!!
	SignerNewTheme {
		KeySetBackupBottomSheet(model,
			mutableStateOf(80.dp),
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
private fun PreviewKeySetBackupFullOverlayScreen() {
	val model = KeySetDetailsModel.createStub().toSeedBackupModel()!!
	SignerNewTheme {
		Box(modifier = Modifier.size(350.dp, 700.dp)) {
			KeySetBackupFullOverlayBottomSheet(model,
				{ _ -> " some long words some some" }, {})
		}
	}
}
