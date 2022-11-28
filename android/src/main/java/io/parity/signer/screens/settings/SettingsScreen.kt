package io.parity.signer.screens

import android.content.res.Configuration
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.SettingsCardTemplate
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.models.AlertState
import io.parity.signer.models.Callback
import io.parity.signer.models.EmptyNavigator
import io.parity.signer.models.Navigator
import io.parity.signer.ui.theme.*
import io.parity.signer.uniffi.Action

/**
 * Settings screen; General purpose stuff like legal info, networks management
 * and history should be here. This is final point in navigation:
 * all subsequent interactions should be in modals or drop-down menus
 */
@Composable
fun SettingsScreen(
	rootNavigator: Navigator,
	isStrongBoxProtected: Boolean,
	appVersion: String,
	wipeToFactory: () -> Unit,
	alertState: State<AlertState?>
) {
	var confirm by remember { mutableStateOf(false) }

	Column(
		verticalArrangement = Arrangement.spacedBy(4.dp)
	) {
		ScreenHeader(
			stringId = R.string.settings_title,
			onBack = { rootNavigator.backAction() },
		)
		SettingsElement(name = stringResource(R.string.settings_networks)) {
			rootNavigator.navigate(Action.MANAGE_NETWORKS)
		}

		Row(Modifier.clickable { rootNavigator.navigate(Action.MANAGE_NETWORKS) }) {
			SettingsCardTemplate(text = stringResource(R.string.settings_networks))
		}
		Row(
			Modifier.clickable {
				if (alertState.value == AlertState.None)
					rootNavigator.navigate(Action.BACKUP_SEED)
				else
					rootNavigator.navigate(Action.SHIELD)
			}
		) {
			SettingsCardTemplate(text = "Backup keys")
		}
		Column(
			Modifier
				.padding(12.dp)
				.clickable { rootNavigator.navigate(Action.VIEW_GENERAL_VERIFIER) }
		) {
			Row {
				Text(
					"Verifier certificate",
					style = MaterialTheme.typography.h1,
					color = MaterialTheme.colors.Text600
				)
				Spacer(Modifier.weight(1f))
			}
		}
		Row(
			Modifier.clickable {
				confirm = true
			}
		) { SettingsCardTemplate(text = "Wipe signer", danger = true) }
		Row(Modifier.clickable { rootNavigator.navigate(Action.SHOW_DOCUMENTS) }) {
			SettingsCardTemplate(text = "About")
		}
		SettingsCardTemplate(
			"Hardware seed protection: $isStrongBoxProtected",
			withIcon = false,
			withBackground = false
		)
		SettingsCardTemplate(
			"Version: $appVersion",
			withIcon = false,
			withBackground = false
		)
	}
}

@Composable
internal fun SettingsElement(name: String, onClick: Callback) {

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
private fun PreviewSettingsScreen() {
	SignerNewTheme {
		val state = remember { mutableStateOf(AlertState.None) }
		SettingsScreen(
			rootNavigator = EmptyNavigator(),
			isStrongBoxProtected = false,
			appVersion = "0.6.1",
			wipeToFactory = {},
			alertState = state,
		)
	}
}
