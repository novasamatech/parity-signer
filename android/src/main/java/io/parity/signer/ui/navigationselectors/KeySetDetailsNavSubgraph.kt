package io.parity.signer.ui.navigationselectors

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.ui.Modifier
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.models.*
import io.parity.signer.screens.keysetdetails.KeySetDetailsScreenFull
import io.parity.signer.screens.keysetdetails.KeySetDetailsScreenView
import io.parity.signer.screens.keysetdetails.backup.SeedBackupFullOverlayBottomSheet
import io.parity.signer.screens.keysetdetails.backup.toSeedBackupModel
import io.parity.signer.screens.keysetdetails.export.KeySetDetailsExportScreenFull

@Composable
fun KeySetDetailsNavSubgraph(
	model: KeySetDetailsModel,
	rootNavigator: Navigator,
	alertState: State<AlertState?>, //for shield icon
	singleton: SignerDataModel,
) {
	val navController = rememberNavController()
	NavHost(
		navController = navController,
		startDestination = KeySetDetailsNavSubgraph.home,
	) {

		composable(KeySetDetailsNavSubgraph.home) {
			KeySetDetailsScreenFull(
				model = model,
				navigator = rootNavigator,
				navController = navController,
				alertState = alertState,
				onRemoveKeySet = {
					val root = model.root
					if (root != null) {
						singleton.removeSeed(root.seedName)
					} else {
						//todo key details check if this functions should be disabled in a first place
						submitErrorState("came to remove key set but root key is not available")
					}
				},
			)
		}
		composable(KeySetDetailsNavSubgraph.multiselect) {
			KeySetDetailsExportScreenFull(
				model = model,
				onClose = { navController.navigate(KeySetDetailsNavSubgraph.home) },
			)
		}
		composable(KeySetDetailsNavSubgraph.backup) {
			//preconditions
			val backupModel = model.toSeedBackupModel()
			if (backupModel == null) {
				submitErrorState("navigated to backup model but without root in KeySet " +
					"it's impossible to backup")
				navController.navigate(KeySetDetailsNavSubgraph.home)
			} else {
				//background
				Box(Modifier.statusBarsPadding()) {
					KeySetDetailsScreenView(
						model = model,
						navigator = EmptyNavigator(),
						alertState = alertState,
						onMenu = {},
					)
				}
				//content
				SeedBackupFullOverlayBottomSheet(
					model = backupModel,
					getSeedPhraseForBackup = singleton::getSeedPhraseForBackup,
					onClose = { navController.navigate(KeySetDetailsNavSubgraph.home) },
				)
			}
		}
	}
}

object KeySetDetailsNavSubgraph {
	const val home = "keyset_details_home"
	const val multiselect = "keyset_details_multiselect"
	const val backup = "keyset_details_backup"
}
