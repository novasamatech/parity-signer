package io.parity.signer.screens.settings.general

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewModelScope
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.components.exposesecurity.ExposedAlert
import io.parity.signer.domain.Callback
import io.parity.signer.screens.error.handleErrorAppState
import io.parity.signer.screens.settings.SettingsNavSubgraph
import io.parity.signer.ui.BottomSheetWrapperRoot
import io.parity.signer.ui.mainnavigation.CoreUnlockedNavSubgraph
import kotlinx.coroutines.launch


@Composable
internal fun SettingsGeneralNavSubgraph(
	coreNavController: NavController,
) {
	val context = LocalContext.current
	val vm: SettingsGeneralViewModel = viewModel()

	val appVersion = rememberSaveable { vm.getAppVersion(context) }
	val shieldState = vm.networkState.collectAsStateWithLifecycle()

	val menuNavController = rememberNavController()

	Box(modifier = Modifier.statusBarsPadding()) {
		SettingsScreenGeneralView(
			navController = coreNavController,
			onWipeData = { menuNavController.navigate(SettingsGeneralMenu.wipe_factory) },
			onOpenLogs = { coreNavController.navigate(SettingsNavSubgraph.logs) },
			onShowTerms = { coreNavController.navigate(SettingsNavSubgraph.terms) },
			onShowPrivacyPolicy = {
				coreNavController.navigate(SettingsNavSubgraph.privacyPolicy)
			},
			onBackup = { coreNavController.navigate(SettingsNavSubgraph.backup) },
			onManageNetworks = {
				coreNavController.navigate(SettingsNavSubgraph.networkList)
			},
			onGeneralVerifier = {
				coreNavController.navigate(SettingsNavSubgraph.generalVerifier)
			},
			onExposedClicked = { menuNavController.navigate(SettingsGeneralMenu.exposed_shield_alert) },
			isStrongBoxProtected = vm.isStrongBoxProtected,
			appVersion = appVersion,
			networkState = shieldState,
		)
	}

	NavHost(
		navController = menuNavController,
		startDestination = SettingsGeneralMenu.empty,
	) {
		val closeAction: Callback = {
			menuNavController.popBackStack()
		}
		composable(SettingsGeneralMenu.empty) {
			//no menu - Spacer element so when other part shown there won't
			// be an appearance animation from top left part despite there shouldn't be
			Spacer(modifier = Modifier.fillMaxSize(1f))
		}
		composable(SettingsGeneralMenu.wipe_factory) {
			BottomSheetWrapperRoot(onClosedAction = closeAction) {
				ConfirmFactorySettingsBottomSheet(
					onCancel = closeAction,
					onFactoryReset = {
						vm.viewModelScope.launch {
							vm.wipeToFactory {
								coreNavController.navigate(
									CoreUnlockedNavSubgraph.KeySet.destination(null)
								)
							}.handleErrorAppState(coreNavController)
						}
					}
				)
			}
		}
		composable(SettingsGeneralMenu.exposed_shield_alert) {
			ExposedAlert(navigateBack = closeAction)
		}
	}
}


private object SettingsGeneralMenu {
	const val empty = "settings_menu_empty"
	const val wipe_factory = "settings_confirm_reset"
	const val exposed_shield_alert = "settings_exposed"
}
