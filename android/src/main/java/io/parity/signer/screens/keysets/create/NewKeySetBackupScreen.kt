package io.parity.signer.screens.keysets.create

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.NotificationFrameTextImportant
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.models.Callback
import io.parity.signer.models.EmptyNavigator
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
	onBack: Callback,
) {

	Column(
		Modifier
			.fillMaxSize(1f)
			.background(MaterialTheme.colors.background),
	) {
		ScreenHeader(
			stringId = R.string.new_key_set_backup_title,
			onBack = onBack,
		)
		Text(
			text = stringResource(R.string.new_key_set_backup_subtitle),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.BodyL,
			modifier = Modifier.padding(horizontal = 24.dp),
		)
		BackupPhraseBox(seedPhrase = model.seedPhrase)
		NotificationFrameTextImportant(messageRes = )
//todo frame like in export keyset
	}
}


/**
 * Local copy of shared [MNewSeedBackup] class
 */
data class NewSeedBackupModel(
	var seed: String,
	var seedPhrase: String,
){
	companion object {
		fun createStub(): NewSeedBackupModel =
			NewSeedBackupModel("seed name", " some long words some some words that consists key phrase")
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
		NewKeySetBackupScreen(model, {})
	}
}
