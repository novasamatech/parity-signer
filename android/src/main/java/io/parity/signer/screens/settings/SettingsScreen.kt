package io.parity.signer.screens.settings

import android.content.res.Configuration
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.*
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ChevronRight
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.alerts.AndroidCalledConfirm
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.components.exposesecurity.ExposedIcon
import io.parity.signer.components.panels.BottomBar2
import io.parity.signer.components.panels.BottomBar2State
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
	wipeToFactory: Callback,
	alertState: State<AlertState?>
) {
	var confirm by remember { mutableStateOf(false) }

	Column(Modifier.background(MaterialTheme.colors.background)) {
		ScreenHeader(stringId = R.string.settings_title)
		Box(modifier = Modifier.weight(1f)) {
			Column(Modifier.verticalScroll(rememberScrollState())) {
				SettingsElement(name = stringResource(R.string.settings_networks)) {
					rootNavigator.navigate(Action.MANAGE_NETWORKS)
				}
				SettingsElement(name = stringResource(R.string.settings_verifier_certificate)) {
					rootNavigator.navigate(Action.VIEW_GENERAL_VERIFIER)
				}
				SettingsElement(name = stringResource(R.string.settings_backup_keys)) {
					if (alertState.value == AlertState.None)
						rootNavigator.navigate(Action.BACKUP_SEED)
					else
						rootNavigator.navigate(Action.SHIELD)
				}
				SettingsElement(name = stringResource(R.string.settings_docs)) {
					rootNavigator.navigate(Action.SHOW_DOCUMENTS)
				}
				SettingsElement(
					name = stringResource(R.string.settings_wipe_data),
					isDanger = true,
				) {
					confirm = true
				}

				Text(
					text = stringResource(
						R.string.settings_hardware_key,
						isStrongBoxProtected.toString()
					),
					style = SignerTypeface.BodyM,
					color = MaterialTheme.colors.textSecondary,
					modifier = Modifier
						.padding(horizontal = 24.dp, vertical = 16.dp)
				)
				Text(
					text = stringResource(R.string.settings_version, appVersion),
					style = SignerTypeface.BodyM,
					color = MaterialTheme.colors.textSecondary,
					modifier = Modifier
						.padding(horizontal = 24.dp)
				)
			}
			ExposedIcon(
				alertState = alertState, navigator = rootNavigator,
				modifier = Modifier
                    .align(Alignment.BottomEnd)
                    .padding(end = 16.dp, bottom = 16.dp)
			)
		}
		BottomBar2(rootNavigator, BottomBar2State.SETTINGS)
	}

	AndroidCalledConfirm(
		show = confirm,
		header = "Wipe ALL data?",
		text = "Factory reset the Signer app. This operation can not be reverted!",
		back = { confirm = false },
		forward = { wipeToFactory() },
		backText = "Cancel",
		forwardText = "Wipe"
	)
}

@Composable
internal fun SettingsElement(
	name: String,
	isDanger: Boolean = false,
	onClick: Callback,
) {
	Row(
		modifier = Modifier
            .clickable(onClick = onClick)
            .padding(vertical = 14.dp),
	) {
		Text(
			text = name,
			style = SignerTypeface.TitleS,
			color = if (isDanger) MaterialTheme.colors.red400 else MaterialTheme.colors.primary,
			modifier = Modifier
                .padding(start = 24.dp)
                .weight(1f)
		)
		Image(
			imageVector = Icons.Filled.ChevronRight,
			contentDescription = null,
			colorFilter = ColorFilter.tint(MaterialTheme.colors.textTertiary),
			modifier = Modifier.padding(horizontal = 16.dp)
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
private fun PreviewSettingsScreen() {
	SignerNewTheme {
		val state = remember { mutableStateOf(AlertState.Past) }
		SettingsScreen(
			rootNavigator = EmptyNavigator(),
			isStrongBoxProtected = false,
			appVersion = "0.6.1",
			wipeToFactory = {},
			alertState = state,
		)
	}
}
