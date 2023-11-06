package io.parity.signer.screens.error.wrongversion

import android.content.res.Configuration
import androidx.activity.compose.BackHandler
import androidx.compose.foundation.layout.Column
import androidx.compose.runtime.Composable
import androidx.compose.ui.tooling.preview.Preview
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.SignerNewTheme

@Composable
fun ErrorWrongUpdateScreen(onBackupClicked: Callback) {
	BackHandler {
		//do nothing
	}
	Column {
		//todo dmitry implement line
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
		SignerNewTheme() {
			ErrorWrongUpdateScreen(
				onBackupClicked = {},
			)
	}
}
