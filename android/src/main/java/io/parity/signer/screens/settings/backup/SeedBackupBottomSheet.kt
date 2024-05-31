package io.parity.signer.screens.settings.backup

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
import io.parity.signer.domain.Callback
import io.parity.signer.screens.keydetails.exportprivatekey.PrivateKeyExportModel
import io.parity.signer.screens.keysetdetails.backup.SeedPhraseBox
import io.parity.signer.ui.BottomSheetWrapperRoot
import io.parity.signer.ui.theme.SignerNewTheme


@Composable
fun SeedBackupFullOverlayBottomSheet(
	seedName: String,
	getSeedPhraseForBackup: suspend (String) -> String?,
	onClose: Callback,
) {
	val timerSize = remember {
		mutableStateOf(80.dp)
	}

	BottomSheetWrapperRoot(onClosedAction = onClose) {
		SeedBackupBottomSheet(seedName, timerSize, getSeedPhraseForBackup, onClose)
	}
	Row(modifier = Modifier.fillMaxSize()) {
		SnackBarCircularCountDownTimer(
			PrivateKeyExportModel.SHOW_PRIVATE_KEY_TIMEOUT,
			stringResource(R.string.seed_backup_countdown_message),
			timerSize,
			Modifier.align(Alignment.Bottom),
			onClose,
		)
	}
}

@Composable
private fun SeedBackupBottomSheet(
	seedName: String,
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
			title = seedName,
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
			Spacer(modifier = Modifier.size(height = timerSize.value, width = 1.dp))
		}
	}

	LaunchedEffect(Unit) {
		val phrase = getSeedPhraseForBackup(seedName)
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
private fun PreviewSeedBackupBottomSheet() {
	SignerNewTheme {
		SeedBackupBottomSheet("seed",
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
	SignerNewTheme {
		Box(modifier = Modifier.size(350.dp, 700.dp)) {
			SeedBackupFullOverlayBottomSheet("seed",
				{ _ -> " some long words some some" }, {})
		}
	}
}
