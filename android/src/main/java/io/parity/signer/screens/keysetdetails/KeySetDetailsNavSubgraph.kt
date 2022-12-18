package io.parity.signer.screens.keysetdetails

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.ui.Modifier
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.models.*
import io.parity.signer.models.storage.removeSeed
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
					singleton.removeSeed(model.root.seedName)
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
				model = model.toSeedBackupModel(),
				getSeedPhraseForBackup = singleton::getSeedPhraseForBackup,
				onClose = { navController.navigate(KeySetDetailsNavSubgraph.home) },
			)
		}
	}
}

object KeySetDetailsNavSubgraph {
	const val home = "keyset_details_home"
	const val multiselect = "keyset_details_multiselect"
	const val backup = "keyset_details_backup"
}
