package io.parity.signer.screens.keysets.create.backupstepscreens

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.NotificationFrameTextImportant
import io.parity.signer.components.base.PrimaryButtonWide
import io.parity.signer.components.base.ScreenHeaderProgressWithButton
import io.parity.signer.domain.Callback
import io.parity.signer.screens.keysetdetails.backup.SeedPhraseBox
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.uniffi.MNewSeedBackup


/**
 * 2/2 stage to create new key set
 * first is NewKeySetNameScreen
 */
@Composable
internal fun NewKeySetBackupScreen(
	seedPhrase: String,
	onProceed: Callback,
	onBack: Callback,
	modifier: Modifier = Modifier,
) {
	Column(
		modifier = modifier
			.fillMaxSize(1f)
			.background(MaterialTheme.colors.background)
			.verticalScroll(rememberScrollState()),
	) {
		ScreenHeaderProgressWithButton(
			canProceed = false,
			currentStep = 2,
			allSteps = 3,
			btnText = stringResource(R.string.button_next),
			onClose = onBack,
			onButton = null,
			backNotClose = true,
		)
		Text(
			text = stringResource(R.string.new_key_set_backup_subtitle),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleL,
			modifier = Modifier
				.padding(horizontal = 24.dp)
				.padding(bottom = 8.dp),
		)
		SeedPhraseBox(seedPhrase = seedPhrase)
		NotificationFrameTextImportant(
			message = stringResource(R.string.new_key_set_backup_warning_message),
			modifier = Modifier
				.padding(horizontal = 16.dp)
		)
		Spacer(modifier = Modifier.weight(1f))

		PrimaryButtonWide(
			label = stringResource(R.string.button_next),
			modifier = Modifier.padding(horizontal = 32.dp, vertical = 24.dp),
			onClicked = onProceed,
		)
	}
}


/**
 * Local copy of shared [MNewSeedBackup] class
 */
data class NewSeedBackupModel(
	var seed: String,
	var seedPhrase: String,
) {
	companion object {
		fun createStub(): NewSeedBackupModel =
			NewSeedBackupModel(
				"seed name",
				" some long words some some words that consists key phrase"
			)
	}
}

fun MNewSeedBackup.toNewSeedBackupModel(): NewSeedBackupModel =
	NewSeedBackupModel(seed = seed, seedPhrase = seedPhrase)


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
private fun PreviewNewKeySetBackupScreen() {
	SignerNewTheme {
		NewKeySetBackupScreen("some words many many words secr fphr phrase", {}, {})
	}
}


@Preview
@Composable
private fun PreviewNewKeySetBackupScreenNarrow() {
	Box(modifier = Modifier.size(height = 400.dp, width = 150.dp)) {
		SignerNewTheme {
			NewKeySetBackupScreen("some words many many words secr fphr phrase", {}, {})
		}
	}
}

@Preview
@Composable
private fun PreviewNewKeySetBackupScreenShort() {
	Box(modifier = Modifier.size(height = 400.dp, width = 200.dp)) {
		SignerNewTheme {
			NewKeySetBackupScreen("some words many many words secr fphr phrase", {}, {})
		}
	}
}
