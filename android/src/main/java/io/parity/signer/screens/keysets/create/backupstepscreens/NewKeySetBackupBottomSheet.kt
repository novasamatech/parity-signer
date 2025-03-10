package io.parity.signer.screens.keysets.create.backupstepscreens

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.CheckboxWithText
import io.parity.signer.components.base.RowButtonsBottomSheet
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.backgroundTertiary

@Composable
internal fun NewKeySetBackupBottomSheet(
	onCancel: Callback,
	onProceed: Callback,
) {
	var confirmBackup by remember { mutableStateOf(false) }
	var confirmNotLoose by remember { mutableStateOf(false) }

	Column(Modifier.background(MaterialTheme.colors.backgroundTertiary))
	{
		Text(
			text = stringResource(R.string.new_key_set_backup_bottomsheet_title),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleL,
			modifier = Modifier
				.padding(start = 24.dp, top = 16.dp, end = 24.dp, bottom = 16.dp)
		)
		CheckboxWithText(
			checked = confirmBackup,
			text = stringResource(R.string.new_key_set_backup_bottomsheet_checkbox_written),
			modifier = Modifier
				.padding(horizontal = 24.dp)
		) { newValue ->
			confirmBackup = newValue
		}
		CheckboxWithText(
			checked = confirmNotLoose,
			text = stringResource(R.string.new_key_set_backup_bottomsheet_checkbox_keep_careful),
			modifier = Modifier
                .padding(horizontal = 24.dp)
                .padding(top = 8.dp)
		) { newValue ->
			confirmNotLoose = newValue
		}
		RowButtonsBottomSheet(
			modifier = Modifier.padding(24.dp),
			labelCancel = stringResource(id = R.string.generic_cancel),
			labelCta = stringResource(id = R.string.button_next),
			onClickedCancel = onCancel,
			onClickedCta = onProceed,
			isCtaEnabled = confirmBackup && confirmNotLoose,
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
private fun PreviewNewKeySetBackupBottomSheet() {
	SignerNewTheme {
		NewKeySetBackupBottomSheet({}, {})
	}
}
