package io.parity.signer.screens.error.wrongversion

import androidx.navigation.NavController
import androidx.navigation.NavGraphBuilder
import androidx.navigation.compose.composable
import androidx.navigation.navigation
import io.parity.signer.ui.mainnavigation.CoreUnlockedNavSubgraph

fun NavGraphBuilder.errorWrongVersionSubgraph(navController: NavController) {
	navigation(
		route = CoreUnlockedNavSubgraph.errorWrongDbVersionUpdate,
		startDestination = ErrorWrongVersionSubgraph.default,
	) {
		composable(ErrorWrongVersionSubgraph.default) {
			ErrorWrongUpdateScreen(onBackupClicked = {
				navController.navigate(
					ErrorWrongVersionSubgraph.backup
				)
			})
		}
		composable(ErrorWrongVersionSubgraph.backup) {
			FallbackRecoverPhraseScreenFull(
				onBack = navController::popBackStack,
				navController = navController,
			)
		}
	}
}

internal object ErrorWrongVersionSubgraph {
	const val default = "error_version_general"
	const val backup = "error_version_backup"
}
