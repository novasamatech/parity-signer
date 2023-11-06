package io.parity.signer.screens.error.wrongversion

import android.content.res.Configuration
import androidx.activity.compose.BackHandler
import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.outlined.SettingsBackupRestore
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.PrimaryButtonWide
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.pink500
import io.parity.signer.ui.theme.textTertiary

@Composable
fun ErrorWrongUpdateScreen(onBackupClicked: Callback) {
	BackHandler {
		//do nothing
	}
	Column(
		modifier = Modifier
			.padding(24.dp)
			.verticalScroll(rememberScrollState())
	) {
		Spacer(Modifier.weight(0.5f))
		Image(
			imageVector = Icons.Outlined.SettingsBackupRestore,
			contentDescription = null,
			colorFilter = ColorFilter.tint(MaterialTheme.colors.pink500),
			modifier = Modifier
				.padding(horizontal = 8.dp)
				.size(80.dp)
				.align(Alignment.CenterHorizontally)
		)
		Spacer(modifier = Modifier.padding(top = 32.dp))
		Text(
			modifier = Modifier
				.fillMaxWidth(1f)
				.padding(horizontal = 8.dp),
			text = stringResource(R.string.error_wrong_version_title),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleL,
			textAlign = TextAlign.Center,
		)
		Spacer(modifier = Modifier.padding(top = 16.dp))
		Text(
			modifier = Modifier
				.fillMaxWidth(1f)
				.padding(horizontal = 8.dp),
			text = stringResource(R.string.error_wrong_version_description),
			color = MaterialTheme.colors.textTertiary,
			style = SignerTypeface.BodyL,
			textAlign = TextAlign.Center,
		)
		Spacer(modifier = Modifier.padding(top = 40.dp))
		PrimaryButtonWide(
			label = stringResource(R.string.error_wrong_version_backup_cta),
			onClicked = onBackupClicked,
		)
		Spacer(Modifier.weight(0.5f))
		//todo dmitry implement
	}
}


@Preview(
	name = "light", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun ErrorWrongUpdateScreenPreview() {
	Box(modifier = Modifier.fillMaxSize()) {
		SignerNewTheme() {
			ErrorWrongUpdateScreen(
				onBackupClicked = {},
			)
		}
	}
}
