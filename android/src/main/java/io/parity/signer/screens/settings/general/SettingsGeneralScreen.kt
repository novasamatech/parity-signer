package io.parity.signer.screens.settings.general

import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ChevronRight
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import androidx.navigation.compose.rememberNavController
import io.parity.signer.R
import io.parity.signer.components.base.ScreenHeaderClose
import io.parity.signer.components.exposesecurity.ExposedIcon
import io.parity.signer.domain.Callback
import io.parity.signer.domain.NetworkState
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.red400
import io.parity.signer.ui.theme.textSecondary
import io.parity.signer.ui.theme.textTertiary


@Composable
internal fun SettingsScreenGeneralView(
	navController: NavController,
	onWipeData: Callback,
	onOpenLogs: Callback,
	onShowTerms: Callback,
	onShowPrivacyPolicy: Callback,
	onBackup: Callback,
	onManageNetworks: Callback,
	onGeneralVerifier: Callback,
	onExposedClicked: Callback,
	isStrongBoxProtected: Boolean,
	appVersion: String,
	networkState: State<NetworkState?>,
) {
	Column(Modifier.background(MaterialTheme.colors.background)) {
		ScreenHeaderClose(
			title = stringResource(R.string.settings_title),
			onClose = { navController.popBackStack() },
		)
		Box(modifier = Modifier.weight(1f)) {
			Column(Modifier.verticalScroll(rememberScrollState())) {
				SettingsElement(
					name = stringResource(R.string.settings_logs),
					onClick = onOpenLogs
				)
				SettingsElement(
					name = stringResource(R.string.settings_networks),
					onClick = onManageNetworks,
				)
				SettingsElement(
					name = stringResource(R.string.settings_verifier_certificate),
					onClick = onGeneralVerifier,
				)
				SettingsElement(
					name = stringResource(R.string.settings_backup),
					onClick = onBackup,
				)
				SettingsElement(
					name = stringResource(R.string.documents_privacy_policy),
					onClick = onShowPrivacyPolicy
				)
				SettingsElement(
					name = stringResource(R.string.documents_terms_of_service),
					onClick = onShowTerms
				)
				SettingsElement(
					name = stringResource(R.string.settings_wipe_data),
					isDanger = true,
					skipChevron = true,
					onClick = onWipeData
				)
				Text(
					text = stringResource(
						R.string.settings_hardware_key, isStrongBoxProtected.toString()
					),
					style = SignerTypeface.BodyM,
					color = MaterialTheme.colors.textSecondary,
					modifier = Modifier.padding(horizontal = 24.dp, vertical = 16.dp)
				)
				Text(
					text = stringResource(R.string.settings_version, appVersion),
					style = SignerTypeface.BodyM,
					color = MaterialTheme.colors.textSecondary,
					modifier = Modifier.padding(horizontal = 24.dp)
				)
			}
			ExposedIcon(
				networkState = networkState,
				onClick = onExposedClicked,
				modifier = Modifier
					.align(Alignment.BottomEnd)
					.padding(end = 16.dp, bottom = 16.dp)
			)
		}
	}
}

@Composable
internal fun SettingsElement(
	name: String,
	isDanger: Boolean = false,
	skipChevron: Boolean = false,
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
		if (!skipChevron) {
			Image(
				imageVector = Icons.Filled.ChevronRight,
				contentDescription = null,
				colorFilter = ColorFilter.tint(MaterialTheme.colors.textTertiary),
				modifier = Modifier.padding(horizontal = 16.dp)
			)
		}
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
		val state = remember { mutableStateOf(NetworkState.Past) }
		SettingsScreenGeneralView(
			navController = rememberNavController(),
			onWipeData = {},
			onOpenLogs = {},
			onShowTerms = {},
			onShowPrivacyPolicy = {},
			onBackup = {},
			onManageNetworks = {},
			onGeneralVerifier = {},
			onExposedClicked = {},
			isStrongBoxProtected = false,
			appVersion = "0.6.1",
			networkState = state,
		)
	}
}
