package io.parity.signer.screens.keysets.restore

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.domain.Navigator
import io.parity.signer.screens.keysets.restore.restorephrase.KeysetRecoverPhraseScreen


@Composable
fun NewKeysetRecoverSecondStepSubgraph(
	rootNavigator: Navigator,
	initialRecoverSeedPhrase: KeysetRecoverModel,
) {
//background
	Box(
		modifier = Modifier
			.fillMaxSize(1f)
			.statusBarsPadding()
			.background(MaterialTheme.colors.background)
	)

	val navController = rememberNavController()
	NavHost(
		navController = navController,
		startDestination = KeysetRecoverSubgraph.KeysetRecoverSeed,
	) {
		composable(KeysetRecoverSubgraph.KeysetRecoverSeed) {
			//todo dmitry viewmodel? add
			KeysetRecoverPhraseScreen(
				rootNavigator, initialRecoverSeedPhrase
			)
			BackHandler(onBack = rootNavigator::backAction)
		}
		composable(KeysetRecoverSubgraph.KeysetRecoverNetworks) {
//			NewKeySetSelectNetworkScreen(//todo dmitry new screen
//				model = model,
//				navigator = rootNavigator,
//				onBack = navController::popBackStack,
//				modifier = Modifier.statusBarsPadding(),
//			)
		}
	}
}

internal object KeysetRecoverSubgraph {
	const val KeysetRecoverSeed = "new_keyset_backup_main"
	const val KeysetRecoverNetworks = "new_keyset_backup_confirmation"
}
