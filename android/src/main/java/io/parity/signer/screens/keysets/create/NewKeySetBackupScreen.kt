package io.parity.signer.screens.keysets.create

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.NotificationFrameTextImportant
import io.parity.signer.components.base.PrimaryButtonWide
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.domain.Callback
import io.parity.signer.screens.keysetdetails.backup.BackupPhraseBox
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.uniffi.MNewSeedBackup

/**
 * 1/2 stage to create new key set
 * second it NewKeySetBackup
 */
@Composable
internal fun NewKeySetBackupScreen(
	model: NewSeedBackupModel,
	onProceed: Callback,
	onBack: Callback,
) {

	Column(
		modifier = Modifier
			.fillMaxSize(1f)
			.background(MaterialTheme.colors.background)
			.verticalScroll(rememberScrollState()),
		horizontalAlignment = Alignment.CenterHorizontally,
	) {
		ScreenHeader(
			title = stringResource(R.string.new_key_set_backup_title),
			onBack = onBack,
		)
		Text(
			text = stringResource(R.string.new_key_set_backup_subtitle),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.BodyL,
			modifier = Modifier
				.padding(horizontal = 24.dp)
				.padding(bottom = 8.dp),
		)
		BackupPhraseBox(seedPhrase = model.seedPhrase)
		NotificationFrameTextImportant(
			message = stringResource(R.string.new_key_set_backup_warning_message),
			modifier = Modifier
				.padding(horizontal = 16.dp)
		)
		Spacer(modifier = Modifier.weight(1f))

		PrimaryButtonWide(
			label = stringResource(R.string.new_key_set_backup_cta),
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
	val model = NewSeedBackupModel(
		"seedname",
		"some words many many words secr fphr phrase"
	)
	SignerNewTheme {
		NewKeySetBackupScreen(model, {}, {})
	}
}


@Preview
@Composable
private fun PreviewNewKeySetBackupScreenNarrow() {
	val model = NewSeedBackupModel(
		"seedname",
		"some words many many words secr fphr phrase"
	)
	Box(modifier = Modifier.size(height = 400.dp, width = 150.dp)) {
		SignerNewTheme {
			NewKeySetBackupScreen(model, {}, {})
		}
	}
}

@Preview
@Composable
private fun PreviewNewKeySetBackupScreenShort() {
	val model = NewSeedBackupModel(
		"seedname",
		"some words many many words secr fphr phrase"
	)
	Box(modifier = Modifier.size(height = 400.dp, width = 200.dp)) {
		SignerNewTheme {
			NewKeySetBackupScreen(model, {}, {})
		}
	}
}
