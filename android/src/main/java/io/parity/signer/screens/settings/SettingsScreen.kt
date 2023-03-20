package io.parity.signer.screens.settings

import android.content.res.Configuration
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
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
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.R
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.components.documents.PpScreen
import io.parity.signer.components.documents.TosScreen
import io.parity.signer.components.exposesecurity.ExposedIcon
import io.parity.signer.components.panels.BottomBar2
import io.parity.signer.components.panels.BottomBar2State
import io.parity.signer.domain.Callback
import io.parity.signer.domain.EmptyNavigator
import io.parity.signer.domain.Navigator
import io.parity.signer.domain.NetworkState
import io.parity.signer.screens.settings.backup.SeedBackupIntegratedScreen
import io.parity.signer.ui.BottomSheetWrapperRoot
import io.parity.signer.ui.theme.*
import io.parity.signer.uniffi.Action

/**
 * Settings screen; General purpose stuff like legal info, networks management
 * and history should be here. This is final point in navigation:
 * all subsequent interactions should be in modals or drop-down menus
 */
@Composable
fun SettingsScreenSubgraph(
	rootNavigator: Navigator,
	isStrongBoxProtected: Boolean,
	appVersion: String,
	wipeToFactory: Callback,
	networkState: State<NetworkState?>
) {
	val navController = rememberNavController()
	NavHost(
		navController = navController,
		startDestination = SettingsScreenSubgraph.home,
	) {

		composable(SettingsScreenSubgraph.home) {
			SettingsScreenGeneralView(
				rootNavigator,
				onWipeData = { navController.navigate(SettingsScreenSubgraph.wipeConformation) },
				onShowTerms = { navController.navigate(SettingsScreenSubgraph.terms) },
				onShowPrivacyPolicy = { navController.navigate(SettingsScreenSubgraph.privacyPolicy) },
				onBackup = { navController.navigate(SettingsScreenSubgraph.backup) },
				isStrongBoxProtected,
				appVersion,
				networkState
			)
		}
		composable(SettingsScreenSubgraph.wipeConformation) {
			SettingsScreenGeneralView(
				rootNavigator,
				onWipeData = { navController.navigate(SettingsScreenSubgraph.wipeConformation) },
				onShowTerms = { navController.navigate(SettingsScreenSubgraph.terms) },
				onShowPrivacyPolicy = { navController.navigate(SettingsScreenSubgraph.privacyPolicy) },
				onBackup = { navController.navigate(SettingsScreenSubgraph.backup) },
				isStrongBoxProtected,
				appVersion,
				networkState
			)

			BottomSheetWrapperRoot(onClosedAction = {
				navController.popBackStack(SettingsScreenSubgraph.home, false)
			}) {
				SettingsWipeAllConfirmation(
					onCancel = {
						navController.popBackStack(
							SettingsScreenSubgraph.home,
							false
						)
					},
					onWipe = wipeToFactory
				)
			}
		}
		composable(SettingsScreenSubgraph.terms) {
			TosScreen(onBack = {
				navController.popBackStack(SettingsScreenSubgraph.home, false)
			})
		}
		composable(SettingsScreenSubgraph.privacyPolicy) {
			PpScreen(onBack = {
				navController.popBackStack(SettingsScreenSubgraph.home, false)
			})
		}
		composable(SettingsScreenSubgraph.backup) {
			SeedBackupIntegratedScreen(rootNavigator)
		}
	}
}

private object SettingsScreenSubgraph {
	const val home = "settings_home"
	const val wipeConformation = "setting_whipe_all_conformation"
	const val terms = "settings_terms_of_service"
	const val privacyPolicy = "settings_privacy_polcy"
	const val backup = "settings_backup"
}

@Composable
private fun SettingsScreenGeneralView(
	rootNavigator: Navigator,
	onWipeData: Callback,
	onShowTerms: Callback,
	onShowPrivacyPolicy: Callback,
	onBackup: Callback,
	isStrongBoxProtected: Boolean,
	appVersion: String,
	networkState: State<NetworkState?>
) {
	Column(Modifier.background(MaterialTheme.colors.background)) {
		ScreenHeader(title = stringResource(R.string.settings_title))
		Box(modifier = Modifier.weight(1f)) {
			Column(Modifier.verticalScroll(rememberScrollState())) {
				SettingsElement(name = stringResource(R.string.settings_networks)) {
					rootNavigator.navigate(Action.MANAGE_NETWORKS)
				}
				SettingsElement(name = stringResource(R.string.settings_verifier_certificate)) {
					rootNavigator.navigate(Action.VIEW_GENERAL_VERIFIER)
				}
				SettingsElement(
					name = stringResource(R.string.settings_backup),
					onClick = onBackup
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
				navigator = rootNavigator,
				modifier = Modifier
					.align(Alignment.BottomEnd)
					.padding(end = 16.dp, bottom = 16.dp)
			)
		}
		BottomBar2(rootNavigator, BottomBar2State.SETTINGS)
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
			rootNavigator = EmptyNavigator(),
			onWipeData = {},
			onShowTerms = {},
			onShowPrivacyPolicy = {},
			onBackup = {},
			isStrongBoxProtected = false,
			appVersion = "0.6.1",
			networkState = state,
		)
	}
}
